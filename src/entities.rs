use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Cohort {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub number: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub cohort_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentScore {
    pub assignment_id: Uuid,
    pub student_id: Uuid,
    pub score: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

