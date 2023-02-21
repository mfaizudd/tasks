use std::sync::Arc;

use sqlx::PgPool;

use crate::{entities::Classroom, ApiError};

pub struct ClassroomService {
    pool: Arc<PgPool>,
}

impl ClassroomService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_classroom_by_invite_code(
        &self,
        invite_code: String,
    ) -> Result<Option<Classroom>, ApiError> {
        let classroom = sqlx::query_as!(
            Classroom,
            "SELECT * FROM classrooms WHERE invite_code = $1",
            invite_code
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(classroom)
    }
}
