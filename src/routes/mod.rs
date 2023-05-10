use crate::handlers::{analytics, root, users, mlb::mlb_player_profile::{mlb_player_handler}, nba::nba_player_profile::{nba_player_handler}};
use axum::{routing::get, Router};


pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::root_handler))
        .route("/nhl", get(analytics::nhl_handler))
        .route("/mlb", get(mlb_player_handler))
        .route("/nba", get(nba_player_handler))
        .route("/users", get(users::users_handler))
}
