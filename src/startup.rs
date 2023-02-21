use axum::{
    routing::{get, post},
    Router,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{net::SocketAddr, sync::Arc};

use crate::{
    config::{DatabaseSettings, Settings},
    routes::{auth, user},
};

pub struct ApiState {
    pub db_pool: Arc<PgPool>,
    pub jwt_secret: Secret<String>,
}

pub async fn run(settings: Settings) -> Result<(), anyhow::Error> {
    let address =
        format!("{}:{}", settings.server.host, settings.server.port).parse::<SocketAddr>()?;
    let db_pool = get_db_pool(settings.database).await?;
    let state = ApiState {
        db_pool: Arc::new(db_pool),
        jwt_secret: settings.server.jwt_secret,
    };
    let auth_routes = Router::new()
        .route("/login/google", post(auth::login_google))
        .route(
            "/register/student/google",
            post(auth::register_student_google),
        )
        .route("/refresh", post(auth::refresh))
        .route("/info", get(auth::info));
    let user_routes = Router::new().route("/", get(user::get_users));
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/auth", auth_routes)
                .nest("/user", user_routes),
        )
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
