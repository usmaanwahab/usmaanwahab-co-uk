use std::fs;
use std::fs::File;
use std::io::Write;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

mod spotify_auth;
use spotify_auth::*;

pub fn get_current_track() -> Result<String, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("spotify_auth.json")?;
    let auth_response: SpotifyAuthResponse = serde_json::from_str(&json_data)?;
    let access_token = auth_response.access_token;
    
    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", access_token);
    headers.insert("Authorization", HeaderValue::from_str(&bearer_token)?);
    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .headers(headers)
        .send()
        .map_err(|e| e.to_string())?;
    let body = response.text().map_err(|e| e.to_string())?;
    Ok(body.to_string())
}
