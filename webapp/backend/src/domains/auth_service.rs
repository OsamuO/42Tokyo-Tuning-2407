use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::web::Bytes;
use fast_image_resize as fir;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use tokio::task;
use log::{error, info};
use redis::AsyncCommands;

use crate::errors::AppError;
use crate::models::user::{Dispatcher, Session, User};
use crate::utils::{generate_session_token, hash_password, verify_password};
use crate::cache::RedisClient;

use super::dto::auth::LoginResponseDto;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_user(&self, username: &str, password: &str, role: &str) -> Result<User, AppError>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn create_dispatcher(&self, user_id: i32, area_id: i32) -> Result<Dispatcher, AppError>;
    async fn find_dispatcher_by_user_id(&self, user_id: i32) -> Result<Option<Dispatcher>, AppError>;
    async fn find_profile_image_name_by_user_id(&self, user_id: i32) -> Result<Option<String>, AppError>;
    async fn create_session(&self, user_id: i32, session_token: &str) -> Result<(), AppError>;
    async fn delete_session(&self, session_token: &str) -> Result<(), AppError>;
    async fn find_session_by_session_token(&self, session_token: &str) -> Result<Session, AppError>;
}

pub struct AuthService<T: AuthRepository> {
    repository: Arc<T>,
    redis_client: Arc<RedisClient>,
}

impl<T: AuthRepository> AuthService<T> {
    pub fn new(repository: T, redis_client: RedisClient) -> Self {
        AuthService {
            repository: Arc::new(repository),
            redis_client: Arc::new(redis_client),
        }
    }

    pub async fn register_user(&self, username: &str, password: &str, role: &str, area: Option<i32>) -> Result<LoginResponseDto, AppError> {
        if role == "dispatcher" && area.is_none() {
            return Err(AppError::BadRequest("Area is required for dispatcher role".into()));
        }

        if let Some(_) = self.repository.find_user_by_username(username).await? {
            return Err(AppError::Conflict("Username already exists".into()));
        }

        let hashed_password = hash_password(password)?;
        let user = self.repository.create_user(username, &hashed_password, role).await?;

        let session_token = generate_session_token();
        self.repository.create_session(user.id, &session_token).await?;

        let login_response = match role {
            "dispatcher" => {
                let dispatcher = self.repository.create_dispatcher(user.id, area.unwrap()).await?;
                LoginResponseDto {
                    user_id: user.id,
                    username: user.username,
                    session_token,
                    role: user.role,
                    dispatcher_id: Some(dispatcher.id),
                    area_id: Some(dispatcher.area_id),
                }
            }
            _ => LoginResponseDto {
                user_id: user.id,
                username: user.username,
                session_token,
                role: user.role,
                dispatcher_id: None,
                area_id: None,
            },
        };

        // Cache user session
        self.cache_session(&session_token, &login_response).await?;

        Ok(login_response)
    }

    pub async fn login_user(&self, username: &str, password: &str) -> Result<LoginResponseDto, AppError> {
        let user = self.repository.find_user_by_username(username).await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

        if !verify_password(&user.password, password)? {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        let session_token = generate_session_token();
        self.repository.create_session(user.id, &session_token).await?;

        let login_response = match user.role.as_str() {
            "dispatcher" => {
                let dispatcher = self.repository.find_dispatcher_by_user_id(user.id).await?
                    .ok_or_else(|| AppError::InternalServerError("Dispatcher not found".into()))?;
                LoginResponseDto {
                    user_id: user.id,
                    username: user.username,
                    session_token,
                    role: user.role,
                    dispatcher_id: Some(dispatcher.id),
                    area_id: Some(dispatcher.area_id),
                }
            }
            _ => LoginResponseDto {
                user_id: user.id,
                username: user.username,
                session_token,
                role: user.role,
                dispatcher_id: None,
                area_id: None,
            },
        };

        // Cache user session
        self.cache_session(&session_token, &login_response).await?;

        Ok(login_response)
    }

    pub async fn logout_user(&self, session_token: &str) -> Result<(), AppError> {
        self.repository.delete_session(session_token).await?;
        self.redis_client.del::<String, ()>(session_token).await?;
        Ok(())
    }

    pub async fn get_resized_profile_image_byte(&self, user_id: i32) -> Result<Bytes, AppError> {
        let profile_image_name = self.repository.find_profile_image_name_by_user_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("Profile image not found".into()))?;

        let path: PathBuf = Path::new(&format!("images/user_profile/{}", profile_image_name)).to_path_buf();

        let img_bytes = tokio::fs::read(&path).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to read image file: {}", e)))?;

        let resized_img = task::spawn_blocking(move || -> Result<Vec<u8>, ImageError> {
            let img = image::load_from_memory(&img_bytes)?;
            let resized = img.resize(500, 500, image::imageops::FilterType::Lanczos3);
            let mut buffer = Vec::new();
            resized.write_to(&mut buffer, ImageOutputFormat::Png)?;
            Ok(buffer)
        }).await
        .map_err(|e| AppError::InternalServerError(format!("Image processing task failed: {}", e)))?
        .map_err(|e| AppError::InternalServerError(format!("Image processing failed: {}", e)))?;

        Ok(Bytes::from(resized_img))
    }

    pub async fn validate_session(&self, session_token: &str) -> Result<bool, AppError> {
        if let Ok(session) = self.get_cached_session(session_token).await {
            return Ok(true);
        }

        let session = self.repository.find_session_by_session_token(session_token).await?;
        Ok(session.is_valid)
    }

    async fn cache_session(&self, session_token: &str, login_response: &LoginResponseDto) -> Result<(), AppError> {
        let serialized = serde_json::to_string(login_response)
            .map_err(|e| AppError::InternalServerError(format!("Failed to serialize session: {}", e)))?;
        
        self.redis_client.set_ex(session_token, &serialized, 3600).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to cache session: {}", e)))?;
        
        Ok(())
    }

    async fn get_cached_session(&self, session_token: &str) -> Result<LoginResponseDto, AppError> {
        let serialized: String = self.redis_client.get(session_token).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get cached session: {}", e)))?;
        
        serde_json::from_str(&serialized)
            .map_err(|e| AppError::InternalServerError(format!("Failed to deserialize session: {}", e)))
    }
}
