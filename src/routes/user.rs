use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use hyper::StatusCode;

use crate::{
    services::UserService,
    startup::ApiState,
    ApiError, response::Response, dto::{PaginationDto, Claims},
};

#[debug_handler]
pub async fn get_users(
    _: Claims,
    State(api_state): State<Arc<ApiState>>,
    Query(pagination): Query<PaginationDto>,
) -> Result<impl IntoResponse, ApiError> {
    let limit = pagination.per_page.unwrap_or(10);
    let offset = (pagination.page.unwrap_or(1) - 1) * limit;
    let order_by = pagination.sort_by.unwrap_or("id".to_string());
    let user_service = UserService::new(api_state.db_pool.clone());
    let users = user_service.get_users(limit, offset, order_by).await?;
    Ok(Response::new(users, "Users fetched successfully".to_string(), vec![]).json(StatusCode::OK))
}
