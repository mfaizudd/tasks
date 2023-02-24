use axum::{routing::get, Router};
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::{DatabaseSettings, OauthSettings, Settings},
    routes,
};

pub struct ApiState {
    pub db_pool: Arc<PgPool>,
    pub oauth_settings: Arc<OauthSettings>,
}

pub async fn run(settings: Settings) -> Result<(), anyhow::Error> {
    let address =
        format!("{}:{}", settings.server.host, settings.server.port).parse::<SocketAddr>()?;
    let db_pool = Arc::new(get_db_pool(settings.database).await?);
    let state = ApiState {
        db_pool: db_pool.clone(),
        oauth_settings: Arc::new(settings.oauth),
    };
    let cohort_routes = Router::new().route("/", get(routes::list_cohorts));
    let app = Router::new()
        .nest("/api/v1", Router::new().nest("/cohorts", cohort_routes))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    println!("Listening on http://{address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub async fn get_db_pool(settings: DatabaseSettings) -> Result<PgPool, anyhow::Error> {
    let options = PgConnectOptions::new()
        .host(&settings.host)
        .port(settings.port)
        .username(&settings.username)
        .password(settings.password.expose_secret())
        .database(&settings.database);
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(options)
        .await?;
    Ok(pool)
}
