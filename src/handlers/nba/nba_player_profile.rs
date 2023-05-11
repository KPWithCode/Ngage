use axum::{http::Response, http::StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

pub async fn nba_player_handler() -> Response<String> {
    match nba_player_data().await {
        Ok(response_body) => {
            let response = Response::new(response_body.clone());
            println!("Response: {:?}", response);
            response
        }
        Err(status) => {
            let error_msg = format!("Error fetching NBA data: {:?}", status);
            Response::builder()
                .status(status)
                .body(error_msg.into())
                .unwrap()
        }
    }
}

async fn nba_player_data() -> Result<String, StatusCode> {
    dotenv::dotenv().ok();
    let nba_player_key = env::var("NBA_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let url = format!(
        "http://api.sportradar.us/nba/trial/v8/en/players/ab532a66-9314-4d57-ade7-bb54a70c65ad/profile.json?api_key={}
        ",
        nba_player_key
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
