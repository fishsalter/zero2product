use axum::routing::{get, post};
use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ping", get(pong))
        .route("/ping", post(pong));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn pong() -> &'static str {
    "pong"
}
