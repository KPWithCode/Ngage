use crate::handlers::{analytics, root};
use axum::{routing::get, routing::post, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::root_handler))
        .route("/analytics", post(analytics::analytics_handler))
}
