use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use serde::{Serialize, Deserialize};
use serde_json;

use base64::Engine;
use base64::engine::general_purpose;

#[derive(Deserialize, Debug)]
pub struct SpotifyCredentials {
    pub client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    scope: Option<String>,
    expires_in: u16,
    refresh_token: String,
}

pub fn refresh_auth() -> Result<String, Box<dyn std::error::Error>> {
    let spotify_credentials = match get_spotify_credentials() {
        Ok(creds) => creds,
        Err(e) => {
            eprintln!("Failed to load spotify credentials: {}", e);
            return Err(e);
        }
    };

    let json_data = fs::read_to_string("spotify_auth.json")?;
    let auth_response: SpotifyAuthResponse = serde_json::from_str(&json_data)?;

    let mut params = HashMap::new();

    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &auth_response.refresh_token);
    params.insert("client_id", &spotify_credentials.client_id);

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .map_err(|e| e.to_string())?;
    let body = response.text().map_err(|e| e.to_string())?;

    let data: SpotifyAuthResponse = serde_json::from_str(&body)?;
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create("spotify_auth.json")?;
    file.write_all(json.as_bytes())?;
    Ok(body.to_string())
}

pub fn get_spotify_credentials() -> Result<SpotifyCredentials, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("config.json")?;
    let credentials: SpotifyCredentials = serde_json::from_str(&json_data)?;
    Ok(credentials)
}

pub fn get_spotify_access_tokens(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let spotify_credentials = match get_spotify_credentials() {
        Ok(creds) => creds,
        Err(e) => {
            eprintln!("Failed to load spotify credentials: {}", e);
            return Err(e);
        }
    };

    let mut params = HashMap::new();

    params.insert("grant_type", "authorization_code");
    params.insert("code", code);
    params.insert("redirect_uri", "https://usmaanwahab.co.uk/callback");

    let mut headers = HeaderMap::new();
    let encoded_auth = general_purpose::STANDARD.encode(&format!(
        "{}:{}",
        spotify_credentials.client_id, spotify_credentials.client_secret
    ));

    let auth_header =
        HeaderValue::from_str(&format!("Basic {}", encoded_auth)).map_err(|e| e.to_string())?;
    headers.insert("Authorization", auth_header);
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .map_err(|e| e.to_string())?;
    let body = response.text().map_err(|e| e.to_string())?;

    let data: SpotifyAuthResponse = serde_json::from_str(&body)?;
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create("spotify_auth.json")?;
    file.write_all(json.as_bytes())?;

    Ok(body)
}

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
