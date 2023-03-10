use axum::{routing::Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// Because main.rs is the crate root adding modules here allow them to be included in the compilation process. These modules are now available through my entire crate.
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
    .nest("/api", routes::routes())
    .layer(cors);


    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on address: {}", addr);

    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Error starting server: {}", e);
    }
}