use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StudentDto {
    pub name: String,
    pub number: String,
    pub cohort_id: Uuid,
}
