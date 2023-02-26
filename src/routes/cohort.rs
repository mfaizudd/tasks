use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    dto::{CohortRequest, PaginationDto, UserInfo},
    entities::{cohort::Cohort, student::Student},
};
use crate::{response::Response, startup::ApiState, ApiError};

pub async fn list_cohorts(
    user: UserInfo,
    Query(pagination): Query<PaginationDto>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohorts = Cohort::find(&state.db_pool, user, pagination).await?;
    Ok(Response::new(cohorts, "Cohorts retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn list_students(
    user: UserInfo,
    Path(cohort_id): Path<Uuid>,
    Query(pagination): Query<PaginationDto>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    if cohort.email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this cohort".to_string(),
        ));
    }
    let students = Student::find_by_cohort(&state.db_pool, cohort_id, pagination).await?;
    Ok(Response::new(students, "Students retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn get_cohort(
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    Ok(Response::new(cohort, "Cohort retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn create_cohort(
    user_info: UserInfo,
    State(state): State<Arc<ApiState>>,
    Json(cohort_request): Json<CohortRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::create(&state.db_pool, user_info, cohort_request.name).await?;
    Ok(Response::new(cohort, "Cohort created".to_string(), vec![]).json(StatusCode::CREATED))
}

pub async fn update_cohort(
    user_info: UserInfo,
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<ApiState>>,
    Json(cohort_request): Json<CohortRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    if cohort.email != user_info.email {
        Err(ApiError::AuthorizationError(
            "You are unauthorized to update this cohort".to_string(),
        ))?;
    }
    let cohort = Cohort::update(&state.db_pool, cohort_id, cohort_request.name).await?;
    Ok(Response::new(cohort, "Cohort updated".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn delete_cohort(
    user_info: UserInfo,
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    if cohort.email != user_info.email {
        Err(ApiError::AuthorizationError(
            "You are unauthorized to update this cohort".to_string(),
        ))?;
    }
    Cohort::delete(&state.db_pool, cohort_id).await?;
    Ok(Response::new((), "Cohort deleted".to_string(), vec![]).json(StatusCode::OK))
}
