use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    dto::{Claims, StudentDto},
    entities::student::Student,
    response::Response,
    startup::ApiState,
    ApiError,
};

pub async fn get_student(
    _: Claims,
    State(api_state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let student = Student::find_one(&api_state.db_pool, id).await?;
    Ok(Response::new(
        student,
        "Student retrieved successfully.".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn create_student(
    _: Claims,
    State(api_state): State<Arc<ApiState>>,
    Json(student): Json<StudentDto>,
) -> Result<impl IntoResponse, ApiError> {
    let student = Student::create(
        &api_state.db_pool,
        student.name,
        student.number,
        student.cohort_id,
    )
    .await?;
    Ok(
        Response::new(student, "Student created successfully.".to_string(), vec![])
            .json(StatusCode::CREATED),
    )
}

pub async fn update_student(
    _: Claims,
    State(api_state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Json(student): Json<StudentDto>,
) -> Result<impl IntoResponse, ApiError> {
    Student::find_one(&api_state.db_pool, id).await?;
    let student = Student::update(
        &api_state.db_pool,
        id,
        student.name,
        student.number,
        student.cohort_id,
    )
    .await?;
    Ok(
        Response::new(student, "Student updated successfully.".to_string(), vec![])
            .json(StatusCode::OK),
    )
}

pub async fn delete_student(
    _: Claims,
    State(api_state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Student::find_one(&api_state.db_pool, id).await?;
    Student::delete(&api_state.db_pool, id).await?;
    Ok(Response::new((), "Student deleted successfully.".to_string(), vec![]).json(StatusCode::OK))
}

