use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserAccount {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub user_type: UserType,
    pub role: UserRole,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Teacher,
    Student,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Email,
    Google,
}

#[derive(Serialize, Deserialize)]
pub struct UserPassword {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Classroom {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub user_id: Uuid,
    pub invite_code: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub score_max: i32,
    pub user_id: Uuid,
    pub classroom_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskAssignment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub score: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
