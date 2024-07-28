use bcrypt::{hash, verify, DEFAULT_COST};
use rand::{Rng, thread_rng};
use crate::errors::AppError;

pub fn generate_session_token() -> String {
    let mut rng = thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            chars[idx] as char
        })
        .collect();
    token
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => Ok(hashed_password),
        Err(_) => Err(AppError::InternalServerError),
    }
}

pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    match verify(input_password, hashed_password) {
        Ok(result) => Ok(result),
        Err(_) => Err(AppError::InternalServerError),
    }
}
