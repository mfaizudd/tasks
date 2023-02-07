use axum::{routing::get, Router};
use std::net::SocketAddr;

pub async fn run(address: &SocketAddr) -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    println!("Listening on http://{address}");
    axum::Server::bind(address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
