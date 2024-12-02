use axum::Router;
use axum::routing::get;
use std::env;
use axum::http::HeaderMap;



async fn health(headers: HeaderMap) -> String {
    format!("Up, answered from 0.0.0.0:{}", env::var("PORT").unwrap_or_else(|_| "".to_string()))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Server running at {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}
