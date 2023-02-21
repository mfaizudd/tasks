use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{
    dto::{AuthRequest, Claims, RefreshRequest, StudentRegisterRequest},
    response::Response,
    services::{AuthService, ClassroomService, UserService},
    startup::ApiState,
    ApiError,
};

#[axum_macros::debug_handler]
pub async fn login_google(
    State(api_state): State<Arc<ApiState>>,
    Json(auth_request): Json<AuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let auth_service = AuthService::new(api_state);
    let auth_response = auth_service
        .login_google(auth_request.access_token.as_str())
        .await?;
    Ok(Response::new(
        auth_response,
        "Authenticated successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn register_student_google(
    State(api_state): State<Arc<ApiState>>,
    Json(register_request): Json<StudentRegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let auth_service = AuthService::new(api_state.clone());
    let classroom_service = ClassroomService::new(api_state.db_pool.clone());
    let invited_classroom = classroom_service
        .get_classroom_by_invite_code(register_request.invite_code)
        .await?;
    invited_classroom.ok_or(ApiError::BadRequest("Invalid invite code".to_string()))?;
    let auth_response = auth_service
        .register_student_google(register_request.access_token.as_str())
        .await?;
    Ok(
        Response::new(auth_response, "Registered successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
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
    Json(refresh_request): Json<RefreshRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let auth_service = AuthService::new(api_state);
    let auth_response = auth_service.refresh(&refresh_request.refresh_token).await?;
    Ok(
        Response::new(auth_response, "Refreshed successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
}
