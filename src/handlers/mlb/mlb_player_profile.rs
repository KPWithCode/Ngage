use axum::{http::Response, http::StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::collections::HashMap;
use std::env;

pub async fn mlb_player_handler() -> Response<String> {
    match mlb_player_data().await {
        Ok(response_data) => {
            let response = Response::new(response_data.to_string());
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
    let player_data: serde_json::Value = serde_json::from_str(&body).map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;


    // Extract the relevant data
    let player_name = player_data["full_name"].as_str().unwrap_or("");
    let batting_stats = &player_data["seasons"][0]["teams"][0]["statistics"]["batting"];
    let ba = batting_stats["avg"].as_str().unwrap_or("N/A");
    let slg = batting_stats["slg"].as_str().unwrap_or("N/A");
    let obp = batting_stats["obp"].as_str().unwrap_or("N/A");
    let pitch_stats = &player_data["seasons"][0]["teams"][0]["statistics"]["pitching"]["overall"];

    // Find the pitch type with the lowest strike percentage
    let mut lowest_strike_pct = f64::MAX;
    let mut best_pitch_type = "";
    if let serde_json::Value::Array(pitches) = pitch_stats {
        let mut pitch_map = HashMap::new();
        for pitch in pitches {
            let pitch_type = pitch["pitch_type"].as_str().unwrap_or("");
            let strikes = pitch["strikes"].as_u64().unwrap_or(0) as f64;
            let total = pitch["total"].as_u64().unwrap_or(0) as f64;
            let strike_pct = strikes / total;
            pitch_map.insert(pitch_type, strike_pct);
        }
        if pitch_map.is_empty() {
            println!("No pitch types found!");
        } else {
            for (pitch_type, strike_pct) in pitch_map {
                if strike_pct < lowest_strike_pct {
                    lowest_strike_pct = strike_pct;
                    best_pitch_type = pitch_type;
                }
            }
        }
    }

    // Construct a new JSON object containing the relevant data
    let response_data = serde_json::json!({
        "Player Name": player_name,
        "BA": ba,
        "SLG": slg,
        "OBP": obp,
        "Best Pitch Type": best_pitch_type,
    });

    Ok(response_data.to_string())
}