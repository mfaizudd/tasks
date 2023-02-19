use std::sync::Arc;

use anyhow::anyhow;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::{
    dto::{AuthResponse, Claims, UserDto, UserInfo, RefreshClaims},
    entities::{UserRole, UserType},
    startup::ApiState,
    ApiError,
};

use super::UserService;

pub struct AuthService {
    api_state: Arc<ApiState>,
}

impl AuthService {
    pub fn new(api_state: Arc<ApiState>) -> Self {
        Self { api_state }
    }

    pub async fn auth_google(&self, token: &str) -> Result<AuthResponse, ApiError> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token)
            .send()
            .await
            .map_err(|_| ApiError::BadRequest("Invalid access token".to_string()))?;

        let user_info = response
            .json::<UserInfo>()
            .await
            .map_err(|_| ApiError::BadRequest("Invalid oauth response".to_string()))?;

        let service = UserService::new(self.api_state.db_pool.clone());
        let user = service.get_user_by_email(user_info.email.clone()).await?;

        let user = match user {
            Some(user) => user,
            None => {
                let user_dto = UserDto {
                    email: user_info.email.clone(),
                    name: user_info.email.clone(),
                    role: UserRole::Student,
                    user_type: UserType::Google,
                };
                let user = service.create_user(user_dto).await?;
                user
            }
        };

        let access_token = jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                sub: user.id.to_string(),
                iat: Utc::now().timestamp(),
                exp: (Utc::now() + Duration::minutes(15)).timestamp(),
            },
            &EncodingKey::from_secret(self.api_state.jwt_secret.expose_secret().as_bytes()),
        )
        .map_err(|_| anyhow!("Unable to create access token"))?;

        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &RefreshClaims {
                sub: user.id.to_string(),
                iat: Utc::now().timestamp(),
                exp: (Utc::now() + Duration::days(30)).timestamp(),
                jti: Uuid::new_v4().to_string(),
            },
            &EncodingKey::from_secret(self.api_state.jwt_secret.expose_secret().as_bytes()),
        )
        .map_err(|_| anyhow!("Unable to create refresh token"))?;
        let auth_response = AuthResponse {
            access_token,
            refresh_token,
        };
        Ok(auth_response)
    }
}
