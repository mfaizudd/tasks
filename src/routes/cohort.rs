use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::dto::{Claims, CohortRequest};
use crate::{entities::Cohort, response::Response, startup::ApiState, ApiError};

pub async fn list_cohorts(
    _: Claims,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohorts = sqlx::query_as!(Cohort, "SELECT * FROM cohorts")
        .fetch_all(&*state.db_pool)
        .await?;
    Ok(Response::new(cohorts, "Cohorts retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn get_cohort(
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = sqlx::query_as!(Cohort, "SELECT * FROM cohorts WHERE id = $1", cohort_id)
        .fetch_one(&*state.db_pool)
        .await?;
    Ok(Response::new(cohort, "Cohort retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn create_cohort(
    Json(cohort_request): Json<CohortRequest>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = sqlx::query_as!(
        Cohort,
        "INSERT INTO cohorts (name, email) VALUES ($1, $2) RETURNING *",
        cohort_request.name,
        cohort_request.email
    )
    .fetch_one(&*state.db_pool)
    .await?;
    Ok(Response::new(cohort, "Cohort created".to_string(), vec![]).json(StatusCode::CREATED))
}

pub async fn update_cohort(
    Path(cohort_id): Path<Uuid>,
    Json(cohort_request): Json<CohortRequest>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = sqlx::query_as!(
        Cohort,
        "UPDATE cohorts SET name = $1, email = $2 WHERE id = $3 RETURNING *",
        cohort_request.name,
        cohort_request.email,
        cohort_id
    )
    .fetch_one(&*state.db_pool)
    .await?;
    Ok(Response::new(cohort, "Cohort updated".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn delete_cohort(
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query!("DELETE FROM cohorts WHERE id = $1", cohort_id)
        .execute(&*state.db_pool)
        .await?;
    Ok(Response::new((), "Cohort deleted".to_string(), vec![]).json(StatusCode::OK))
}
