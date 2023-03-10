use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    dto::{StudentRequest, UserInfo},
    entities::{cohort::Cohort, student::Student},
    response::Response,
    startup::AppState,
    ApiError,
};

pub async fn get_student(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let student = Student::find_one(&state.db_pool, id).await?;
    if student.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this student".to_string(),
        ));
    }
    Ok(Response::new(
        student,
        "Student retrieved successfully.".to_string(),
        vec![],
    )
    .json(StatusCode::OK))
}

pub async fn create_student(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Json(student): Json<StudentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, student.cohort_id).await?;
    if cohort.email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to create a student in this cohort".to_string(),
        ));
    }
    let student = Student::create(
        &state.db_pool,
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
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<StudentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let student = Student::find_one(&state.db_pool, id).await?;
    if student.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to update this student".to_string(),
        ));
    }
    let student = Student::update(
        &state.db_pool,
        id,
        input.name,
        input.number,
        input.cohort_id,
    )
    .await?;
    Ok(
        Response::new(student, "Student updated successfully.".to_string(), vec![])
            .json(StatusCode::OK),
    )
}

pub async fn delete_student(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let student = Student::find_one(&state.db_pool, id).await?;
    if student.cohort_email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to delete this student".to_string(),
        ));
    }
    Student::delete(&state.db_pool, id).await?;
    Ok(Response::new((), "Student deleted successfully.".to_string(), vec![]).json(StatusCode::OK))
}
