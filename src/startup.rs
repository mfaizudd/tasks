use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::{config::OauthSettings, routes::auth};

pub async fn run(address: &SocketAddr, settings: OauthSettings) -> Result<(), anyhow::Error> {
    let oauth_client = auth::oauth_client(settings);
    let auth_routes = Router::new()
        .route("/google", get(auth::google_auth))
        .route("/callback", get(auth::login_callback));
    let app = Router::new()
        .nest("/api", Router::new().nest("/auth", auth_routes))
        .with_state(oauth_client);

    println!("Listening on http://{address}");
    axum::Server::bind(address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
