use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{PaginationDto, StudentRequest};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub number: String,
    pub cohort_email: String,
    pub cohort_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Student {
    pub async fn find(db: &PgPool, pagination: PaginationDto) -> Result<Vec<Student>, sqlx::Error> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT
                s.id,
                s.name,
                number,
                c.email as cohort_email,
                cohort_id,
                s.created_at,
                s.updated_at
            FROM students s
            JOIN cohorts c ON s.cohort_id = c.id
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
            SELECT
                s.id,
                s.name,
                number,
                c.email as cohort_email,
                cohort_id,
                s.created_at,
                s.updated_at
            FROM students s
            JOIN cohorts c ON s.cohort_id = c.id
            WHERE cohort_id = $1
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
            SELECT
                s.id,
                s.name,
                number,
                c.email as cohort_email,
                cohort_id,
                s.created_at,
                s.updated_at
            FROM students s
            JOIN cohorts c ON s.cohort_id = c.id
            WHERE s.id = $1
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
            WITH s AS (
                INSERT INTO students (name, number, cohort_id)
                VALUES ($1, $2, $3)
                RETURNING *
            ) SELECT
                s.id,
                s.name,
                number,
                c.email as cohort_email,
                cohort_id,
                s.created_at,
                s.updated_at
            FROM s
            JOIN cohorts c ON s.cohort_id = c.id
            "#,
            name,
            number,
            cohort_id
        )
        .fetch_one(db)
        .await?;

        Ok(student)
    }

    pub async fn batch_create(
        db: &PgPool,
        students: Vec<StudentRequest>,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let mut tx = db.begin().await?;

        let mut students = students
            .into_iter()
            .map(|student| {
                sqlx::query_as!(
                    Student,
                    r#"
                    WITH s AS (
                        INSERT INTO students (name, number, cohort_id)
                        VALUES ($1, $2, $3)
                        RETURNING *
                    ) SELECT
                        s.id,
                        s.name,
                        number,
                        c.email as cohort_email,
                        cohort_id,
                        s.created_at,
                        s.updated_at
                    FROM s
                    JOIN cohorts c ON s.cohort_id = c.id
                    "#,
                    student.name,
                    student.number,
                    student.cohort_id
                )
            })
            .collect::<Vec<_>>();

        while let Some(student) = students.pop() {
            student.fetch_one(&mut tx).await?;
        }

        tx.commit().await?;

        Ok(vec![])
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
            WITH s AS (
                UPDATE students
                SET name = $1, number = $2, cohort_id = $3
                WHERE id = $4
                RETURNING *
            ) SELECT
                s.id,
                s.name,
                number,
                c.email as cohort_email,
                cohort_id,
                s.created_at,
                s.updated_at
            FROM s
            JOIN cohorts c ON s.cohort_id = c.id
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

    pub async fn delete(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM students
            WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
