use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;

use crate::{
    dto::{AssignmentRequest, PaginationDto, UserInfo},
    entities::{assignment::Assignment, cohort::Cohort, score::Score},
    response::Response,
    startup::AppState,
    ApiError,
};

pub async fn list_assignments(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationDto>,
) -> Result<impl IntoResponse, ApiError> {
    let assignments = Assignment::find(&state.db_pool, user.email, pagination).await?;
    Ok(Response::new(
        assignments,
        "Assignments retrieved successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn get_assignment(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this assignment".to_string(),
        ));
    }
    Ok(Response::new(
        assignment,
        "Assignment retrieved successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn list_assignment_scores(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this assignment".to_string(),
        ));
    }
    let scores = Score::find_by_asssignment(&state.db_pool, assignment_id).await?;
    Ok(
        Response::new(scores, "Scores retrieved successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
}

pub async fn create_assignment(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Json(assignment): Json<AssignmentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, assignment.cohort_id).await?;
    if cohort.email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to create a new assignment in this cohort".to_string(),
        ));
    }
    let assignment = Assignment::create(&state.db_pool, assignment).await?;
    Ok(Response::new(
        assignment,
        "Assignment created successfully".to_string(),
        vec![],
    )
    .json(StatusCode::CREATED))
}

pub async fn update_assignment(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<uuid::Uuid>,
    Json(input): Json<AssignmentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to update this assignment".to_string(),
        ));
    }
    let assignment = Assignment::update(&state.db_pool, assignment_id, input).await?;
    Ok(Response::new(
        assignment,
        "Assignment updated successfully".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn delete_assignment(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to delete this assignment".to_string(),
        ));
    }
    Assignment::delete(&state.db_pool, assignment_id).await?;
    Ok(
        Response::new((), "Assignment deleted successfully".to_string(), vec![])
            .json(StatusCode::OK),
    )
}
