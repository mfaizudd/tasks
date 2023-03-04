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
    pub cohort_email: String,
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
        email: String,
        pagination: PaginationDto,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
            WHERE c.email = $1
            ORDER BY $2 DESC
            LIMIT $3
            OFFSET $4
            "#,
            email,
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
            SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
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
            SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
            WHERE a.id IN (
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
            SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
            WHERE a.id = $1
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
            WITH a AS (
                INSERT INTO assignments (name, description, cohort_id)
                VALUES ($1, $2, $3)
                RETURNING *
            ) SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM a
            JOIN cohorts c ON c.id = a.cohort_id
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
            WITH a AS (
                UPDATE assignments
                SET name = $1, description = $2, cohort_id = $3
                WHERE id = $4
                RETURNING *
            ) SELECT
                a.id,
                a.name,
                c.email as cohort_email,
                description,
                cohort_id,
                a.created_at,
                a.updated_at
            FROM a
            JOIN cohorts c ON c.id = a.cohort_id
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
