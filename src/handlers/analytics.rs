use axum::http::{Response, StatusCode};
use std::convert::Infallible;

pub async fn analytics_handler() -> Result<Response<String>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("Real-time analytics data".to_string())
        .unwrap())
}
