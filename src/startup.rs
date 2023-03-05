use axum::{
    http::HeaderValue,
    routing::{delete, get, post, put},
    Router,
};
use hyper::{header, Method};
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::{DatabaseSettings, OauthSettings, RedisSettings, ServerSettings, Settings},
    redis::RedisPool,
    routes,
};

pub struct AppState {
    pub db_pool: Arc<PgPool>,
    pub oauth_settings: Arc<OauthSettings>,
    pub redis_pool: Arc<RedisPool>,
}

pub async fn run(settings: Settings) -> Result<(), anyhow::Error> {
    let address =
        format!("{}:{}", settings.server.host, settings.server.port).parse::<SocketAddr>()?;
    let db_pool = Arc::new(get_db_pool(settings.database).await?);
    let redis_pool = Arc::new(get_redis_pool(settings.redis).await?);
    let state = AppState {
        db_pool: db_pool.clone(),
        oauth_settings: Arc::new(settings.oauth),
        redis_pool: redis_pool.clone(),
    };
    sqlx::migrate!().run(&*db_pool).await?;
    let cors_layer = get_cors_layer(settings.server);
    let cohort_routes = Router::new()
        .route("/", get(routes::list_cohorts))
        .route("/:id", get(routes::get_cohort))
        .route("/", post(routes::create_cohort))
        .route("/:id", put(routes::update_cohort))
        .route("/:id", delete(routes::delete_cohort))
        .route("/:id/students", get(routes::list_cohort_students))
        .route("/:id/students/upload", post(routes::upload_students))
        .route("/:id/assignments", get(routes::list_cohort_assignments));
    let student_routes = Router::new()
        .route("/:id", get(routes::get_student))
        .route("/", post(routes::create_student))
        .route("/:id", put(routes::update_student))
        .route("/:id", delete(routes::delete_student));
    let assignment_routes = Router::new()
        .route("/", get(routes::list_assignments))
        .route("/:id", get(routes::get_assignment))
        .route("/", post(routes::create_assignment))
        .route("/:id", put(routes::update_assignment))
        .route("/:id", delete(routes::delete_assignment))
        .route("/:id/scores", get(routes::list_assignment_scores));
    let score_routes = Router::new()
        .route("/", get(routes::get_score))
        .route("/", put(routes::save_score)) // Upsert
        .route("/", delete(routes::delete_score));
    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/cohorts", cohort_routes)
                .nest("/students", student_routes)
                .nest("/assignments", assignment_routes)
                .nest("/scores", score_routes),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
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
    tracing::info!("Connecting to Postgres...");
    tracing::info!("Options: {:?}", options);
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(options);
    Ok(pool)
}

pub async fn get_redis_pool(settings: RedisSettings) -> Result<RedisPool, anyhow::Error> {
    let cfg =
        deadpool_redis::Config::from_url(format!("redis://{}:{}", settings.host, settings.port));
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
    Ok(pool)
}

pub fn get_cors_layer(settings: ServerSettings) -> CorsLayer {
    let cors_layer = CorsLayer::new()
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT_LANGUAGE,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(
            settings
                .allowed_origins
                .iter()
                .map(|s| s.parse::<HeaderValue>().unwrap())
                .collect::<Vec<HeaderValue>>(),
        );
    cors_layer
}
