use axum::{
    extract::ConnectInfo,
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn greet(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {}", addr.ip())
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    println!("Starting Rust server on port 8080");

    let app = Router::new()
        .route("/", get(greet))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
