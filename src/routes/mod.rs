use crate::handlers::{analytics, root, users};
use axum::{routing::get, Router};


pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::root_handler))
        .route("/nhl", get(analytics::nhl_handler))
        .route("/users", get(users::users_handler))
}
