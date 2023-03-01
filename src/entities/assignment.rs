use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{AssignmentRequest, PaginationDto};

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

impl Assignment {
    pub async fn find(
        db: &PgPool,
        pagination: PaginationDto,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            ORDER BY $1 DESC
            LIMIT $2
            OFFSET $3
            "#,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;
        Ok(assignments)
    }

    pub async fn find_by_cohort(
        db: &PgPool,
        cohort_id: Uuid,
        pagination: PaginationDto,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE cohort_id = $1
            ORDER BY $2 DESC
            LIMIT $3
            OFFSET $4
            "#,
            cohort_id,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;
        Ok(assignments)
    }

    pub async fn find_by_student(
        db: &PgPool,
        student_id: Uuid,
        pagination: PaginationDto,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE id IN (
                SELECT assignment_id FROM assignment_scores
                WHERE student_id = $1
            )
            ORDER BY $2 DESC
            LIMIT $3
            OFFSET $4
            "#,
            student_id,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;
        Ok(assignments)
    }

    pub async fn find_one(
        db: &PgPool,
        assignment_id: Uuid,
    ) -> Result<Assignment, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE id = $1
            "#,
            assignment_id
        )
        .fetch_one(db)
        .await?;
        Ok(assignment)
    }

    pub async fn create(
        db: &PgPool,
        request: AssignmentRequest,
    ) -> Result<Assignment, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            INSERT INTO assignments (name, description, cohort_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            request.name,
            request.description,
            request.cohort_id
        )
        .fetch_one(db)
        .await?;
        Ok(assignment)
    }

    pub async fn update(
        db: &PgPool,
        assignment_id: Uuid,
        request: AssignmentRequest,
    ) -> Result<Assignment, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            UPDATE assignments
            SET name = $1, description = $2, cohort_id = $3
            WHERE id = $4
            RETURNING *
            "#,
            request.name,
            request.description,
            request.cohort_id,
            assignment_id
        )
        .fetch_one(db)
        .await?;
        Ok(assignment)
    }

    pub async fn delete(db: &PgPool, assignment_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM assignments
            WHERE id = $1
            "#,
            assignment_id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
