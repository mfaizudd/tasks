use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::PaginationDto;

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub number: String,
    pub cohort_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Student {
    pub async fn find(db: &PgPool, pagination: PaginationDto) -> Result<Vec<Student>, sqlx::Error> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT * FROM students
            ORDER BY $1 DESC
            LIMIT $2 OFFSET $3
            "#,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;

        Ok(students)
    }

    pub async fn find_by_cohort(
        db: &PgPool,
        cohort_id: Uuid,
        pagination: PaginationDto,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT * FROM students WHERE cohort_id = $1
            ORDER BY $2 DESC
            LIMIT $3 OFFSET $4
            "#,
            cohort_id,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;

        Ok(students)
    }

    pub async fn find_one(db: &PgPool, id: Uuid) -> Result<Student, sqlx::Error> {
        let student = sqlx::query_as!(
            Student,
            r#"
            SELECT * FROM students WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await?;

        Ok(student)
    }

    pub async fn create(
        db: &PgPool,
        name: String,
        number: String,
        cohort_id: Uuid,
    ) -> Result<Student, sqlx::Error> {
        let student = sqlx::query_as!(
            Student,
            r#"
            INSERT INTO students (name, number, cohort_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            name,
            number,
            cohort_id
        )
        .fetch_one(db)
        .await?;

        Ok(student)
    }

    pub async fn update(
        db: &PgPool,
        id: Uuid,
        name: String,
        number: String,
        cohort_id: Uuid,
    ) -> Result<Student, sqlx::Error> {
        let student = sqlx::query_as!(
            Student,
            r#"
            UPDATE students
            SET name = $1, number = $2, cohort_id = $3
            WHERE id = $4
            RETURNING *
            "#,
            name,
            number,
            cohort_id,
            id
        )
        .fetch_one(db)
        .await?;

        Ok(student)
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> Result<Student, sqlx::Error> {
        let student = sqlx::query_as!(
            Student,
            r#"
            DELETE FROM students
            WHERE id = $1
            RETURNING *
            "#,
            id
        )
        .fetch_one(db)
        .await?;

        Ok(student)
    }
}
