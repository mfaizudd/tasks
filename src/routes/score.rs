use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;

use crate::{
    dto::{ScoreQuery, ScoreRequest, UserInfo},
    entities::{assignment::Assignment, score::Score, student::Student},
    response::Response,
    startup::AppState,
    ApiError,
};

#[axum_macros::debug_handler]
pub async fn get_score(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ScoreQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, query.assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this score.".into(),
        ));
    }
    let score = Score::find_one(&state.db_pool, query.assignment_id, query.student_id).await?;
    Ok(Response::new(score, "Score retrieved successfully.".into(), vec![]).json(StatusCode::OK))
}

pub async fn save_score(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ScoreRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, body.assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to update this score.".into(),
        ));
    }
    Student::find_one(&state.db_pool, body.student_id).await?;
    let score = Score::upsert(&state.db_pool, body).await?;
    Ok(Response::new(score, "Score updated successfully.".into(), vec![]).json(StatusCode::OK))
}

pub async fn delete_score(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ScoreQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let assignment = Assignment::find_one(&state.db_pool, query.assignment_id).await?;
    if assignment.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to delete this score.".into(),
        ));
    }
    Student::find_one(&state.db_pool, query.student_id).await?;
    Score::delete(&state.db_pool, query.assignment_id, query.student_id).await?;
    Ok(Response::new((), "Score deleted successfully.".into(), vec![]).json(StatusCode::OK))
}
