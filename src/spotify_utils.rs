use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json;
use std::collections::HashMap;

use crate::spotify_auth::{read_spotify_auth, refresh_spotify_auth};

pub fn get_current_track() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match refresh_spotify_auth() {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e.to_string()),
    };
    let spotify_auth_response = match read_spotify_auth() {
        Ok(spotify_auth) => spotify_auth,
        Err(e) => {
            eprintln!("Failed to read spotify auth data: {}", e);
            return Err(e);
        }
    };

    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", &spotify_auth_response.access_token);
    headers.insert("Authorization", HeaderValue::from_str(&bearer_token)?);
    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .headers(headers)
        .send()?;

    // Spotify is not playing
    if response.status() == 204 {
        return Ok(serde_json::from_str("{}").unwrap());
    }

    let body = response.text()?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}

pub fn get_top_items(
    item_type: &str,
    term: &str,
    limit: u16,
    offset: u16,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match refresh_spotify_auth() {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e.to_string()),
    };
    let spotify_auth_response = match read_spotify_auth() {
        Ok(spotify_auth) => spotify_auth,
        Err(e) => {
            eprintln!("Failed to read spotify auth data: {}", e);
            return Err(e);
        }
    };

    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", &spotify_auth_response.access_token);
    headers.insert("Authorization", HeaderValue::from_str(&bearer_token)?);

    let limit_str = limit.to_string();
    let offset_str = offset.to_string();

    let mut params = HashMap::new();
    params.insert("term", term);
    params.insert("limit", &limit_str);
    params.insert("offset", &offset_str);

    let client = Client::new();
    let response = client
        .get(format!("https://api.spotify.com/v1/me/top/{}", item_type))
        .query(&params)
        .headers(headers)
        .send()?;

    let body = response.text()?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}
