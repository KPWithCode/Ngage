use axum::{http::Response, http::StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

pub async fn mlb_player_handler() -> Response<String> {
    match mlb_player_data().await {
        Ok(response_body) => {
            let response = Response::new(response_body.clone());
            println!("Response: {:?}", response);
            response
        }
        Err(status) => {
            let error_msg = format!("Error fetching MLB data: {:?}", status);
            Response::builder()
                .status(status)
                .body(error_msg.into())
                .unwrap()
        }
    }
}

async fn mlb_player_data() -> Result<String, StatusCode> {
    dotenv::dotenv().ok();
    let mlb_player_profile = env::var("MLB_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let url = format!(
        "http://api.sportradar.us/mlb/trial/v7/en/players/46734ad0-e55b-4e2f-8a0d-72387470fcdf/profile.json?api_key={}",
        mlb_player_profile
    );
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("reqwest"));
    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let status = res.status();
    let bytes = res
        .bytes()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = String::from_utf8(bytes.to_vec()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(body)
}
