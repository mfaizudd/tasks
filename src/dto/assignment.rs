use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AssignmentRequest {
    pub name: String,
    pub description: String,
    pub cohort_id: Uuid,
}

#[derive(Deserialize)]
pub struct AssignmentScoreRequest {
    pub assignment_id: Uuid,
    pub student_id: Uuid,
    pub score: i32,
}
