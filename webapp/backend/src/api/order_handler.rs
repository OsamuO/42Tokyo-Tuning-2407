use crate::domains::dto::order::{
    ClientOrderRequestDto, DispatcherOrderRequestDto, UpdateOrderStatusRequestDto,
};
use crate::domains::order_service::OrderService;
use crate::errors::AppError;
use crate::repositories::auth_repository::AuthRepositoryImpl;
use crate::repositories::map_repository::MapRepositoryImpl;
use crate::repositories::order_repository::OrderRepositoryImpl;
use crate::repositories::tow_truck_repository::TowTruckRepositoryImpl;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

pub async fn update_order_status_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
    req: web::Json<UpdateOrderStatusRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service.update_order_status(req.order_id, &req.status).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}

pub async fn get_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    match service.find_order_with_details_by_id(path.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(err) => Err(err),
    }
}

#[derive(Deserialize, Debug)]
pub struct PaginatedOrderQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    status: Option<String>,
    area: Option<i32>,
}

pub async fn get_paginated_orders_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
    query: web::Query<PaginatedOrderQuery>,
) -> Result<HttpResponse, AppError> {
    match service
        .get_paginated_orders(
            query.page.unwrap_or(0),
            query.page_size.unwrap_or(10),
            query.sort_by.clone(),
            query.sort_order.clone(),
            query.status.clone(),
            query.area,
        )
        .await
    {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        Err(err) => Err(err),
    }
}

pub async fn create_client_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
    req: web::Json<ClientOrderRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service
        .create_client_order(req.client_id, req.node_id, req.car_value)
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err),
    }
}

pub async fn create_dispatcher_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
    req: web::Json<DispatcherOrderRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service
        .create_dispatcher_order(
            req.order_id,
            req.dispatcher_id,
            req.tow_truck_id,
            req.order_time,
        )
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}
