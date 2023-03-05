use std::sync::Arc;

use axum::{
    extract::{Json, Multipart, Path, Query, State},
    response::IntoResponse,
};
use csv::Reader;
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    dto::{CohortRequest, PaginationDto, StudentRequest, UserInfo},
    entities::{assignment::Assignment, cohort::Cohort, student::Student},
};
use crate::{response::Response, startup::AppState, ApiError};

pub async fn list_cohorts(
    user: UserInfo,
    Query(pagination): Query<PaginationDto>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohorts = Cohort::find(&state.db_pool, user, pagination).await?;
    Ok(Response::new(cohorts, "Cohorts retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn list_cohort_students(
    user: UserInfo,
    Path(cohort_id): Path<Uuid>,
    Query(pagination): Query<PaginationDto>,
    State(state): State<Arc<AppState>>,
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

pub async fn list_cohort_assignments(
    user: UserInfo,
    Path(cohort_id): Path<Uuid>,
    Query(pagination): Query<PaginationDto>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    if cohort.email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to view this cohort".to_string(),
        ));
    }
    let assignments = Assignment::find_by_cohort(&state.db_pool, cohort_id, pagination).await?;
    Ok(
        Response::new(assignments, "Assignments retrieved".to_string(), vec![])
            .json(StatusCode::OK),
    )
}

pub async fn get_cohort(
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    Ok(Response::new(cohort, "Cohort retrieved".to_string(), vec![]).json(StatusCode::OK))
}

pub async fn create_cohort(
    user_info: UserInfo,
    State(state): State<Arc<AppState>>,
    Json(cohort_request): Json<CohortRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::create(&state.db_pool, user_info, cohort_request.name).await?;
    Ok(Response::new(cohort, "Cohort created".to_string(), vec![]).json(StatusCode::CREATED))
}

pub async fn upload_students(
    user: UserInfo,
    State(state): State<Arc<AppState>>,
    Path(cohort_id): Path<Uuid>,
    mut payload: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let cohort = Cohort::find_one(&state.db_pool, cohort_id).await?;
    if cohort.email != user.email {
        return Err(ApiError::AuthorizationError(
            "You are not authorized to upload students to this cohort".to_string(),
        ));
    }
    let mut students: Vec<StudentRequest> = vec![];
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|_| ApiError::BadRequest("Error parsing payload".to_string()))?
    {
        let name = field
            .name()
            .ok_or_else(|| ApiError::BadRequest("Error parsing payload".to_string()))?;
        if name != "students" {
            return Err(ApiError::BadRequest("Invalid field name".to_string()));
        }
        let bytes = field
            .bytes()
            .await
            .map_err(|_| ApiError::BadRequest("Error parsing payload".to_string()))?;
        let mut reader = Reader::from_reader(bytes.as_ref());
        while let Some(record) = reader.records().next() {
            let record =
                record.map_err(|_| ApiError::BadRequest("Error parsing payload".to_string()))?;
            if record.len() != 2 {
                return Err(ApiError::BadRequest("Invalid number of fields".to_string()));
            }
            let student = StudentRequest {
                cohort_id,
                number: record[0].to_string(),
                name: record[1].to_string(),
            };
            students.push(student);
        }
    }
    let students = Student::batch_create(&state.db_pool, students).await?;
    Ok(
        Response::new(students, "Students created successfully".into(), vec![])
            .json(StatusCode::CREATED),
    )
}

pub async fn update_cohort(
    user_info: UserInfo,
    Path(cohort_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
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
    State(state): State<Arc<AppState>>,
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
