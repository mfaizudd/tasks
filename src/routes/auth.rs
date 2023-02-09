use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use base64::{engine::general_purpose, Engine};
use chrono::{Utc, Duration};
use hyper::StatusCode;
use jsonwebtoken::{EncodingKey, Header};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{
    config::OauthSettings,
    entities::{UserRole, UserType},
    services::{dto::UserDto, UserService},
    startup::ApiState,
    ApiError, response::Response,
};

pub fn oauth_client(settings: OauthSettings) -> Result<BasicClient, anyhow::Error> {
    Ok(BasicClient::new(
        ClientId::new(settings.client_id),
        Some(ClientSecret::new(settings.client_secret)),
        AuthUrl::new(settings.authorization_url)?,
        Some(TokenUrl::new(settings.token_url)?),
    )
    .set_redirect_uri(RedirectUrl::new(settings.redirect_url)?))
}

#[derive(Deserialize)]
pub struct OauthParams {
    redirect_url: String,
}

#[derive(Serialize, Deserialize)]
struct OauthState {
    redirect_url: String,
    csrf_token: CsrfToken,
}

pub async fn google_auth(
    State(api_state): State<Arc<ApiState>>,
    Query(params): Query<OauthParams>,
) -> Result<impl IntoResponse, ApiError> {
    let state = OauthState {
        redirect_url: params.redirect_url,
        csrf_token: CsrfToken::new_random(),
    };
    let state = serde_json::to_string(&state)?;
    let state = general_purpose::URL_SAFE_NO_PAD.encode(state);
    let state = CsrfToken::new(state);

    let (auth_url, _csrf_token) = api_state
        .oauth_client
        .authorize_url(|| state)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .url();
    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
pub struct UserInfo {
    email: String,
    email_verified: bool,
    picture: String,
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
}

#[axum_macros::debug_handler]
pub async fn login_callback(
    State(api_state): State<Arc<ApiState>>,
    Query(request): Query<AuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let state = general_purpose::URL_SAFE_NO_PAD
        .decode(request.state)
        .map_err(|_| ApiError::BadRequest("Invalid state".to_string()))?;
    let state = serde_json::from_str::<OauthState>(
        &String::from_utf8(state).map_err(|_| ApiError::BadRequest("Invalid state".to_string()))?,
    )?;
    let client = api_state.oauth_client.clone();
    let token = client
        .exchange_code(AuthorizationCode::new(request.code))
        .request_async(async_http_client)
        .await
        .map_err(|_| ApiError::BadRequest("Invalid code".to_string()))?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|_| ApiError::BadRequest("Invalid code".to_string()))?;

    let user_info = response
        .json::<UserInfo>()
        .await
        .map_err(|_| ApiError::BadRequest("Invalid code".to_string()))?;

    let service = UserService::new(api_state.db_pool.clone());
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

    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: user.id.to_string(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() +Duration::hours(24)).timestamp(),
        },
        &EncodingKey::from_secret(api_state.jwt_secret.expose_secret().as_bytes()),
    ).map_err(|_| anyhow!("Unable to create jwt"))?;
    println!("{}", state.redirect_url);
    Ok(Response::new(token, "Authenticated successfully".to_string(), vec![]).json(StatusCode::OK))
}
