use axum::{http::Response, http::StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

pub async fn nhl_handler() -> Response<String> {
    match nhl_data().await {
        Ok(response_body) => {
            let response = Response::new(response_body.clone());
            println!("Response: {:?}", response);
            response
        }
        Err(status) => {
            let error_msg = format!("Error fetching NHL data: {:?}", status);
            Response::builder()
                .status(status)
                .body(error_msg.into())
                .unwrap()
        }
    }
}

async fn nhl_data() -> Result<String, StatusCode> {
    dotenv::dotenv().ok();
    let nhl_key = env::var("NHL_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let url = format!(
        "http://api.sportradar.us/nhl/trial/v7/en/games/2023/05/04/schedule.json?api_key={}",
        nhl_key
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
