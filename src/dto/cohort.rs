use serde::Deserialize;

#[derive(Deserialize)]
pub struct CohortRequest {
    pub name: String,
}
