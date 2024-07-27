use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::mysql::MySqlPool;
use crate::domains::dto::order::OrderWithDetails; 

use super::{
    auth_service::AuthRepository,
    dto::order::{CompletedOrderDto, OrderDto},
    map_service::MapRepository,
    tow_truck_service::TowTruckRepository,
};
use crate::{
    errors::AppError,
    models::order::{CompletedOrder, Order},
};

pub trait OrderRepository {
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError>;
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError>;
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError>;
    async fn create_order(
        &self,
        customer_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError>;
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError>;
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError>;
    async fn get_all_completed_orders(&self) -> Result<Vec<CompletedOrder>, AppError>;
    fn get_pool(&self) -> &MySqlPool;
}

#[derive(Debug)]
pub struct OrderService<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> {
    order_repository: T,
    tow_truck_repository: U,
    auth_repository: V,
    map_repository: W,
}

impl<
        T: OrderRepository + std::fmt::Debug,
        U: TowTruckRepository + std::fmt::Debug,
        V: AuthRepository + std::fmt::Debug,
        W: MapRepository + std::fmt::Debug,
    > OrderService<T, U, V, W>
{
    pub fn new(
        order_repository: T,
        tow_truck_repository: U,
        auth_repository: V,
        map_repository: W,
    ) -> Self {
        OrderService {
            order_repository,
            tow_truck_repository,
            auth_repository,
            map_repository,
        }
    }

    pub async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        self.order_repository
            .update_order_status(order_id, status)
            .await
    }

    pub async fn find_order_with_details_by_id(&self, id: i32) -> Result<OrderWithDetails, AppError> {
        let order_with_details = sqlx::query_as!(
            OrderWithDetails,
            r#"
            SELECT 
                o.id,
                o.client_id,
                c.username AS client_username,
                o.dispatcher_id,
                d.user_id AS dispatcher_user_id,
                du.username AS dispatcher_username,
                o.tow_truck_id,
                t.driver_id AS driver_user_id,
                tu.username AS driver_username,
                o.node_id,
                n.area_id, 
                o.car_value,
                o.status,
                o.order_time AS "order_time: NaiveDateTime",
                o.completed_time AS "completed_time: NaiveDateTime"
            FROM orders o
            LEFT JOIN users c ON o.client_id = c.id
            LEFT JOIN dispatchers d ON o.dispatcher_id = d.id
            LEFT JOIN users du ON d.user_id = du.id
            LEFT JOIN tow_trucks t ON o.tow_truck_id = t.id
            LEFT JOIN users tu ON t.driver_id = tu.id
            JOIN nodes n ON o.node_id = n.id
            WHERE o.id = ?
            "#,
            id
        )
        .fetch_one(self.order_repository.get_pool()) // order_repositoryのpoolを使用
        .await?;

        Ok(order_with_details)
    }

    pub async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderDto>, AppError> {
        let orders = self
            .order_repository
            .get_paginated_orders(page, page_size, sort_by, sort_order, status, area)
            .await?;

        let mut results = Vec::new();

        for order in orders {
            let client_username = self
                .auth_repository
                .find_user_by_id(order.client_id)
                .await
                .unwrap()
                .unwrap()
                .username;

            let dispatcher = match order.dispatcher_id {
                Some(dispatcher_id) => self
                    .auth_repository
                    .find_dispatcher_by_id(dispatcher_id)
                    .await
                    .unwrap(),
                None => None,
            };

            let (dispatcher_user_id, dispatcher_username) = match dispatcher {
                Some(dispatcher) => (
                    Some(dispatcher.user_id),
                    Some(
                        self.auth_repository
                            .find_user_by_id(dispatcher.user_id)
                            .await
                            .unwrap()
                            .unwrap()
                            .username,
                    ),
                ),
                None => (None, None),
            };

            let tow_truck = match order.tow_truck_id {
                Some(tow_truck_id) => self
                    .tow_truck_repository
                    .find_tow_truck_by_id(tow_truck_id)
                    .await
                    .unwrap(),
                None => None,
            };

            let (driver_user_id, driver_username) = match tow_truck {
                Some(tow_truck) => (
                    Some(tow_truck.driver_id),
                    Some(
                        self.auth_repository
                            .find_user_by_id(tow_truck.driver_id)
                            .await
                            .unwrap()
                            .unwrap()
                            .username,
                    ),
                ),
                None => (None, None),
            };

            let order_area_id = self
                .map_repository
                .get_area_id_by_node_id(order.node_id)
                .await
                .unwrap();

            results.push(OrderDto {
                id: order.id,
                client_id: order.client_id,
                client_username: Some(client_username),
                dispatcher_id: order.dispatcher_id,
                dispatcher_user_id,
                dispatcher_username,
                tow_truck_id: order.tow_truck_id,
                driver_user_id,
                driver_username,
                area_id: order_area_id,
                status: order.status,
                node_id: order.node_id,
                car_value: order.car_value,
                order_time: order.order_time,
                completed_time: order.completed_time,
            });
        }

        Ok(results)
    }

    pub async fn create_client_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        match self
            .order_repository
            .create_order(client_id, node_id, car_value)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::BadRequest),
        }
    }

    pub async fn create_dispatcher_order(
        &self,
        order_id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
        order_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        if (self
            .order_repository
            .create_completed_order(order_id, tow_truck_id, order_time)
            .await)
            .is_err()
        {
            return Err(AppError::BadRequest);
        }

        self.order_repository
            .update_order_dispatched(order_id, dispatcher_id, tow_truck_id)
            .await?;

        self.tow_truck_repository
            .update_status(tow_truck_id, "busy")
            .await?;

        Ok(())
    }

    pub async fn get_completed_orders(&self) -> Result<Vec<CompletedOrderDto>, AppError> {
        let orders = self.order_repository.get_all_completed_orders().await?;
        let order_dtos = orders
            .into_iter()
            .map(CompletedOrderDto::from_entity)
            .collect();

        Ok(order_dtos)
    }
}
