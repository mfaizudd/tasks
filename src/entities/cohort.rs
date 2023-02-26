use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{PaginationDto, UserInfo};

#[derive(Serialize, Deserialize)]
pub struct Cohort {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Cohort {
    pub async fn find(
        db: &PgPool,
        user: UserInfo,
        pagination: PaginationDto,
    ) -> Result<Vec<Cohort>, sqlx::Error> {
        let cohorts = sqlx::query_as!(
            Cohort,
            r#"
            SELECT * FROM cohorts
            WHERE email = $1
            ORDER BY $2 ASC
            LIMIT $3
            OFFSET $4
            "#,
            user.email,
            pagination.order_by(),
            pagination.limit(),
            pagination.offset()
        )
        .fetch_all(db)
        .await?;
        Ok(cohorts)
    }

    pub async fn find_one(db: &PgPool, id: Uuid) -> Result<Cohort, sqlx::Error> {
        let cohort = sqlx::query_as!(
            Cohort,
            r#"
            SELECT * FROM cohorts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(cohort)
    }

    pub async fn create(db: &PgPool, user: UserInfo, name: String) -> Result<Cohort, sqlx::Error> {
        let cohort = sqlx::query_as!(
            Cohort,
            r#"
            INSERT INTO cohorts (name, email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            name,
            user.email
        )
        .fetch_one(db)
        .await?;
        Ok(cohort)
    }

    pub async fn update(db: &PgPool, id: Uuid, name: String) -> Result<Cohort, sqlx::Error> {
        let cohort = sqlx::query_as!(
            Cohort,
            r#"
            UPDATE cohorts
            SET name = $1
            WHERE id = $2
            RETURNING *
            "#,
            name,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(cohort)
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM cohorts
            WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
