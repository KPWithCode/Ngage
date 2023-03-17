use crate::handlers::{analytics, root, users};
use axum::{routing::get, routing::post, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::root_handler))
        .route("/analytics", post(analytics::analytics_handler))
        .route("/users", get(users::users_handler))
}
