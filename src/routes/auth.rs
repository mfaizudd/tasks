use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use base64::{engine::general_purpose, Engine};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::config::OauthSettings;

pub fn oauth_client(settings: OauthSettings) -> BasicClient {
    BasicClient::new(
        ClientId::new(settings.client_id),
        Some(ClientSecret::new(settings.client_secret)),
        AuthUrl::new(settings.authorization_url).unwrap(),
        Some(TokenUrl::new(settings.token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(settings.redirect_url).unwrap())
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
    State(client): State<BasicClient>,
    Query(params): Query<OauthParams>,
) -> impl IntoResponse {
    // let server_redirect_url = format!(
    //     "{}{}",
    //     client.redirect_url().unwrap().as_str(),
    //     params.redirect_url
    // );
    let state = OauthState {
        redirect_url: params.redirect_url,
        csrf_token: CsrfToken::new_random(),
    };
    let state = serde_json::to_string(&state).unwrap();
    let state = general_purpose::URL_SAFE_NO_PAD.encode(state);
    let state = CsrfToken::new(state);

    let (auth_url, _csrf_token) = client
        // .set_redirect_uri(RedirectUrl::new(server_redirect_url).unwrap())
        .authorize_url(|| state)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .url();

    Redirect::to(auth_url.as_str())
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    // redirect_url: String,
    code: String,
    state: String,
}

pub async fn login_callback(
    State(client): State<BasicClient>,
    Query(request): Query<AuthRequest>,
) -> impl IntoResponse {
    let state = general_purpose::URL_SAFE_NO_PAD
        .decode(request.state)
        .unwrap();
    let state = serde_json::from_str::<OauthState>(&String::from_utf8(state).unwrap()).unwrap();
    let token = client
        .exchange_code(AuthorizationCode::new(request.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap();

    let user_info: serde_json::Value = response.json().await.unwrap();

    format!(
        "Hello {}! Your email is {}, and you will be redirected to {}",
        user_info["name"], user_info["email"], state.redirect_url
    )
}
