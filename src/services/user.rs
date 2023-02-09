use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{entities::UserAccount, ApiError};

use super::dto::UserDto;

pub struct UserService {
    pool: Arc<PgPool>,
}

impl UserService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_users(
        &self,
        limit: i64,
        offset: i64,
        sort: String,
    ) -> Result<Vec<UserAccount>, ApiError> {
        let user = sqlx::query_as!(
            UserAccount,
            r#"
            SELECT 
                id, 
                name,
                email, 
                type as "user_type: _",
                role as "role: _",
                created_at, 
                updated_at
            FROM user_accounts
            ORDER BY $3
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset,
            sort
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserAccount, ApiError> {
        let user = sqlx::query_as!(
            UserAccount,
            r#"
            SELECT 
                id, 
                name,
                email, 
                type as "user_type: _",
                role as "role: _",
                created_at, 
                updated_at
            FROM user_accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<UserAccount>, ApiError> {
        let user = sqlx::query_as!(
            UserAccount,
            r#"
            SELECT 
                id, 
                name,
                email, 
                type as "user_type: _",
                role as "role: _",
                created_at, 
                updated_at
            FROM user_accounts
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserDto) -> Result<UserAccount, ApiError> {
        let user = sqlx::query_as!(
            UserAccount,
            r#"
            INSERT INTO user_accounts (name, email, type, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, type as "user_type: _", role as "role: _", created_at, updated_at
            "#,
            user.name,
            user.email,
            user.user_type as _,
            user.role as _
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, user: UserDto) -> Result<UserAccount, ApiError> {
        let user = sqlx::query_as!(
            UserAccount,
            r#"
            UPDATE user_accounts
            SET name = $1, email = $2, type = $3, role = $4, updated_at = now()
            WHERE id = $5
            RETURNING id, name, email, type as "user_type: _", role as "role: _", created_at, updated_at
            "#,
            user.name,
            user.email,
            user.user_type as _,
            user.role as _,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), ApiError> {
        let _ = sqlx::query!("SELECT id FROM user_accounts WHERE id = $1", id)
            .fetch_one(&*self.pool)
            .await?;
        sqlx::query!(
            r#"
            DELETE FROM user_accounts
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
