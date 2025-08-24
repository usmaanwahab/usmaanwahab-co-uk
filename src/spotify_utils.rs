use serde_json;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::spotify_auth::{
    read_spotify_auth
};

pub fn get_current_track() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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

pub fn get_top_items(item_type: &str, range: &str, limit: u8, offset: u8) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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
        .get("https://api.spotify.com/v1/me/top/artists")
        .headers(headers)
        .send()?;

    let body = response.text()?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}
