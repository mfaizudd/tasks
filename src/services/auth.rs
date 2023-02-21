use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::{
    dto::{AuthResponse, Claims, RefreshClaims, UserDto, UserInfo},
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

    async fn verify_google_user(&self, token: &str) -> Result<UserInfo, ApiError> {
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

        Ok(user_info)
    }

    fn generate_access_token(&self, user_id: String) -> String {
        let claims = Claims {
            sub: user_id,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::minutes(15)).timestamp(),
        };

        let key = self.api_state.jwt_secret.expose_secret().as_bytes();
        jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
            .expect("Failed to generate access token")
    }

    fn decode_token<T: DeserializeOwned>(&self, refresh_token: &str) -> Result<T, ApiError> {
        let key = &DecodingKey::from_secret(self.api_state.jwt_secret.expose_secret().as_bytes());
        let claims = jsonwebtoken::decode::<T>(refresh_token, key, &Validation::default())
            .map_err(|_| ApiError::AuthorizationError("Unauthorized".to_string()))?
            .claims;
        Ok(claims)
    }

    fn generate_refresh_token(&self, user_id: String) -> String {
        let claims = RefreshClaims {
            sub: user_id,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::days(7)).timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        let key = self.api_state.jwt_secret.expose_secret().as_bytes();
        jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
            .expect("Failed to generate refresh token")
    }

    pub async fn login_google(&self, token: &str) -> Result<AuthResponse, ApiError> {
        let user_info = self.verify_google_user(token).await?;
        let service = UserService::new(self.api_state.db_pool.clone());
        let user = service.get_user_by_email(user_info.email.clone()).await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(ApiError::AuthorizationError("User not found".to_string()));
            }
        };

        let access_token = self.generate_access_token(user.id.to_string());
        let refresh_token = self.generate_refresh_token(user.id.to_string());
        let auth_response = AuthResponse {
            access_token,
            refresh_token,
        };
        Ok(auth_response)
    }

    pub async fn register_student_google(&self, token: &str) -> Result<AuthResponse, ApiError> {
        let user_info = self.verify_google_user(token).await?;
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

        let access_token = self.generate_access_token(user.id.to_string());
        let refresh_token = self.generate_refresh_token(user.id.to_string());
        let auth_response = AuthResponse {
            access_token,
            refresh_token,
        };
        Ok(auth_response)
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<AuthResponse, ApiError> {
        let claims = self.decode_token::<RefreshClaims>(refresh_token)?;
        let access_token = self.generate_access_token(claims.sub.clone());
        let refresh_token = self.generate_refresh_token(claims.sub);
        let auth_response = AuthResponse {
            access_token,
            refresh_token,
        };
        Ok(auth_response)
    }

    pub async fn verify_access_token(&self, token: &str) -> Result<Claims, ApiError> {
        let claims = self.decode_token::<Claims>(token)?;
        Ok(claims)
    }
}
