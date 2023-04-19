use axum::http::{Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use std::convert::Infallible;

pub async fn dk_handler() -> Result<Response<String>, Box<dyn std::error::Error>> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let dkKey = "";

    let url = format!(
        "https://api.draftkings.com/partner/v1/gameplayerlps.json?date={}&format=json",
        date
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-rapidapi-host",
        HeaderValue::from_str("api.draftkings.com").unwrap(),
    );
    headers.insert("x-rapidapi-key", HeaderValue::from_str(dkKey).unwrap());

    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to get DraftKings data".to_string())
            .unwrap());
    }

    let response_text = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&response_text)?;
    // let player_props_array = match json["playerProps"].as_array() {
    //     Some(array) => array,
    //     None => &vec![],
    // };
    // let player_props = player_props_array.to_vec();
    let player_props:Vec<String> = Vec::new();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(format!("DraftKings data for {}: {:?}", date, player_props))
        .unwrap())
}


pub async fn fanduel_handler() -> Result<Response<String>, Infallible> {
    let api_key = "YOUR_API_KEY_HERE";
    let api_secret = "YOUR_API_SECRET_HERE";

    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Key", HeaderValue::from_str(api_key).unwrap());
    headers.insert("X-Auth-Secret", HeaderValue::from_str(api_secret).unwrap());

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.fanduel.com/realtime/analytics")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(res)
        .unwrap())
}
