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

use crate::{startup::ApiState, ApiError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
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
        let key = &DecodingKey::from_secret(state.jwt_secret.expose_secret().as_bytes());
        let token = jsonwebtoken::decode::<Claims>(bearer.token(), key, &Validation::default())
            .map_err(|_| reject())?;
        Ok(token.claims)
    }
}
