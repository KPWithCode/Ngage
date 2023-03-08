use crate::handlers::analytics;

pub fn routes() -> axum::Router {

    axum::Router::new().route("/analytics", axum::routing::post(analytics::analytics_handler))
}