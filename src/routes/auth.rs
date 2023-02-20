use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{
    dto::{AuthRequest, Claims},
    response::Response,
    services::{AuthService, UserService},
    startup::ApiState,
    ApiError,
};

#[axum_macros::debug_handler]
pub async fn google(
    State(api_state): State<Arc<ApiState>>,
    Json(auth_request): Json<AuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let auth_service = AuthService::new(api_state);
    let auth_response = auth_service
        .auth_google(auth_request.refresh_token.as_str())
        .await?;
    Ok(Response::new(
        auth_response,
        "Authenticated successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn info(
    claims: Claims,
    State(api_state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let user_service = UserService::new(api_state.db_pool.clone());
    let user = user_service
        .get_user(
            claims
                .sub
                .parse()
                .map_err(|_| ApiError::BadRequest("Invalid user id".to_string()))?,
        )
        .await?;
    Ok(Response::new(user, "User retrieved successfully".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn refresh(
    State(api_state): State<Arc<ApiState>>,
    Json(auth_request): Json<AuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let auth_service = AuthService::new(api_state);
    let auth_response = auth_service.refresh(&auth_request.refresh_token).await?;
    Ok(
        Response::new(auth_response, "Refreshed successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
}
