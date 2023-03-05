use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::ScoreRequest;

#[derive(Serialize)]
pub struct Score {
    pub assignment_id: Uuid,
    pub assignment_name: String,
    pub cohort_id: Uuid,
    pub cohort_name: String,
    pub student_id: Uuid,
    pub student_name: String,
    pub score: Option<i32>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Score {
    pub async fn find_by_asssignment(
        pool: &PgPool,
        assignment_id: Uuid,
    ) -> Result<Vec<Score>, sqlx::Error> {
        let scores = sqlx::query_as!(
            Score,
            r#"
            SELECT
                a.id AS assignment_id,
                a.name AS assignment_name,
                c.id AS cohort_id,
                c.name AS cohort_name,
                s.id AS student_id,
                s.name AS student_name,
                sc.score AS "score?",
                sc.created_at as "created_at?",
                sc.updated_at as "updated_at?"
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
            JOIN students s ON s.cohort_id = c.id
            LEFT JOIN assignment_scores sc ON sc.student_id = s.id AND sc.assignment_id = a.id
            WHERE a.id = $1
            "#,
            assignment_id
        )
        .fetch_all(pool)
        .await?;

        Ok(scores)
    }

    pub async fn find_one(
        pool: &PgPool,
        assignment_id: Uuid,
        student_id: Uuid,
    ) -> Result<Score, sqlx::Error> {
        let score = sqlx::query_as!(
            Score,
            r#"
            SELECT
                a.id AS assignment_id,
                a.name AS assignment_name,
                c.id AS cohort_id,
                c.name AS cohort_name,
                s.id AS student_id,
                s.name AS student_name,
                sc.score AS "score?",
                sc.created_at as "created_at?",
                sc.updated_at as "updated_at?"
            FROM assignments a
            JOIN cohorts c ON c.id = a.cohort_id
            JOIN students s ON s.cohort_id = c.id
            LEFT JOIN assignment_scores sc ON sc.student_id = s.id AND sc.assignment_id = a.id
            WHERE a.id = $1 AND s.id = $2
            "#,
            assignment_id,
            student_id
        )
        .fetch_one(pool)
        .await?;

        Ok(score)
    }

    pub async fn upsert(pool: &PgPool, score: ScoreRequest) -> Result<Score, sqlx::Error> {
        let score = sqlx::query_as!(
            Score,
            r#"
            WITH sc AS (
                INSERT INTO assignment_scores (assignment_id, student_id, score)
                VALUES ($1, $2, $3)
                ON CONFLICT (assignment_id, student_id) DO UPDATE
                SET score = $3
                RETURNING *
            ) SELECT
                a.id AS assignment_id,
                a.name AS assignment_name,
                c.id AS cohort_id,
                c.name AS cohort_name,
                s.id AS student_id,
                s.name AS student_name,
                sc.score AS "score?",
                sc.created_at as "created_at?",
                sc.updated_at as "updated_at?"
            FROM sc
            JOIN assignments a ON a.id = sc.assignment_id
            JOIN cohorts c ON c.id = a.cohort_id
            JOIN students s ON s.id = sc.student_id
            "#,
            score.assignment_id,
            score.student_id,
            score.score
        )
        .fetch_one(pool)
        .await?;

        Ok(score)
    }

    pub async fn delete(
        pool: &PgPool,
        assignment_id: Uuid,
        student_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM assignment_scores WHERE assignment_id = $1 AND student_id = $2"#,
            assignment_id,
            student_id
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
