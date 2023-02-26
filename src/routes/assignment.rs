use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;

use crate::{
    dto::{AssignmentRequest, Claims},
    entities::assignment::Assignment,
    response::Response,
    startup::ApiState,
    ApiError,
};

pub async fn get_assignment(
    _: Claims,
    State(state): State<Arc<ApiState>>,
    Path(assignment_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let assignments = Assignment::find_one(&state.db_pool, assignment_id).await?;
    Ok(Response::new(
        assignments,
        "Assignment retrieved successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn create_assignment(
    _: Claims,
    State(state): State<Arc<ApiState>>,
    Json(assignment): Json<AssignmentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::create(&state.db_pool, assignment).await?;
    Ok(Response::new(
        assignment,
        "Assignment created successfully".to_string(),
        vec![],
    )
    .json(StatusCode::CREATED))
}

pub async fn update_assignment(
    _: Claims,
    State(state): State<Arc<ApiState>>,
    Path(assignment_id): Path<uuid::Uuid>,
    Json(assignment): Json<AssignmentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Assignment::find_one(&state.db_pool, assignment_id).await?;
    let assignment = Assignment::update(&state.db_pool, assignment_id, assignment).await?;
    Ok(Response::new(
        assignment,
        "Assignment updated successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn delete_assignment(
    _: Claims,
    State(state): State<Arc<ApiState>>,
    Path(assignment_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Assignment::find_one(&state.db_pool, assignment_id).await?;
    Assignment::delete(&state.db_pool, assignment_id).await?;
    Ok(
        Response::new((), "Assignment deleted successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
}
