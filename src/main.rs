use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new().route("/", get(root)).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on address: {}", addr);

    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Error starting server: {}", e);
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
