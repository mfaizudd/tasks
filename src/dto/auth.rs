use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{auth::verify_access_token, startup::ApiState, ApiError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub auth_time: i64,
    pub acr: String,
    pub amr: Vec<String>,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct StudentRegisterRequest {
    pub access_token: String,
    pub invite_code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
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
        let jwks = reqwest::get(&state.oauth_settings.jwks_url)
            .await
            .map_err(|_| reject())?
            .json::<JwkSet>()
            .await
            .map_err(|_| reject())?;
        let header = decode_header(bearer.token()).map_err(|_| reject())?;
        let kid = header.kid.ok_or_else(reject)?;
        let jwk = jwks.find(&kid).ok_or_else(reject)?;
        let claims = match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key =
                    DecodingKey::from_rsa_components(&rsa.n, &rsa.e).map_err(|_| reject())?;
                let validation = Validation::new(jwk.common.algorithm.unwrap());
                let claims = decode::<Claims>(bearer.token(), &decoding_key, &validation)
                    .map_err(|_| reject())?
                    .claims;
                claims
            }
            _ => return Err(reject()),
        };
        println!("{:?}", claims);
        Ok(claims)
    }
}
