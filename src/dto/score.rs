use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ScoreRequest {
    pub assignment_id: Uuid,
    pub student_id: Uuid,
    pub score: i32,
}

#[derive(Deserialize)]
pub struct ScoreQuery {
    pub assignment_id: Uuid,
    pub student_id: Uuid,
}
