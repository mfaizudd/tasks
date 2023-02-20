use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};
use jsonwebtoken::{DecodingKey, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{startup::ApiState, ApiError, services::AuthService};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
    pub jti: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct UserInfo {
    pub email: String,
    pub email_verified: bool,
    pub picture: String,
    pub sub: String,
}


#[async_trait]
impl FromRequestParts<Arc<ApiState>> for Claims {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ApiState>,
    ) -> Result<Self, Self::Rejection> {
        let reject = || ApiError::AuthorizationError("Unauthorized".to_string());
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| reject())?;
        let auth_service = AuthService::new(state.clone());
        let claims = auth_service
            .verify_access_token(bearer.token())
            .await
            .map_err(|_| reject())?;
        Ok(claims)
    }
}
