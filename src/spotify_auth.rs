use std::collection::HashMap;

use std::fs;
use std::fs::File;
use std::io::Write;

use serde::{Serialize, Deserialize};
use serde_json;

use reqwest::blocking::client;
use reqwest::header::{HeaderMap, HeaderValue};

use base64::Engine;
use base64::engine::general_purpose;

const CONFIG_PATH: &str = "config.json";
const REDIRECT_URI: &str = "https://usmaanwahab.co.uk/callback";

const TOKEN_ENDPOINT: &str = "https://accounts.spotify.com/api/token";

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyAuthCredentials {
    pub client_id: String,
    pub client_secret: String
};

#[derive(Serialize, Deserialize, Debug)]
pub SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    scope: Option<String>,
    expires_in: u16,
    refresh_token: String
};


pub fn read_spotify_credentials() -> Result<SpotifyAuthCredentials, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(CONFIG_PATH)?;
    let spotify_credentials: SpotifyAuthCredentials = serde_json::from_str(&json_data)?;
    Ok(spotify_credentials)
}

pub fn request_spotify_access_token(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let spotify_credentials = match read_spotify_credentials() {
        Ok(credentials) = credentials;
        Err(e) => {
            eprintln!("Failed to load spotify credentials: {}", e);
            return Err(e);
        }
    };

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", code);
    params.insert("redirect_uri", REDIRECT_URI);

    let encoded_secret_and_id = general_purpose::STANDARD.encode(&format!(
            "{}:{}",
            spotify_credentials.client_id, spotify_credentials.client_secret
    ));

    let auth_header = HeaderValue::from_str(
        &format!("Basic {}", encoded_secret_and_id))?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header);
    headers.insert("Content-Type", HeaderValue::from_static("applications/x-www-form-urlencoded"));

    let client = Client::new();
    let response = client
        .post(TOKEN_ENDPOINT)
        .header(headers)
        .form(&params)
        .send()?;

    let body = response.text()?;
    let json_data: SpotifyAuthResponse = serde_json::from_str(&body);
    let formatted_json = serde_json::to_string_pretty(&data);
    let mut file = File::create(SPOTIFY_AUTH_PATH);
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn read_spotify_auth() -> Result<SpotifyAuthResponse, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(SPOTIFY_AUTH_PATH)?;
    let spotify_auth_response: SpotifyAuthResponse = serde_json::from_str(&json_data)?;
    Ok(spotify_auth_response)
}

pub fn refresh_spotify_auth() -> Result<(), Box<dyn std::error::Error>> {
    let spotify_credentials = match get_spotify_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("Failed to read spotify response: {}", e);
            return Err(e);
        }
    };
    
    let spotify_auth_response = match read_spotify_auth() {
        Ok(spotify_auth) => spotift_auth,
        Err(e) => {
            eprintln!("Failed to read spotify auth data: {}", e);
            return Err(e);
        }
    };

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &spotify_auth_response.refresh_token);
    
    let mut headers = HeaderMap::new();

    let encoded_secret_and_id = general_purpose::STANDARD.encode(&format!(
            "{}:{}",
            spotify_credentials.client_id, spotify_credentials.client_secret
    ));

    let auth_header = HeaderValue::from_str(
        &format!("Basic {}", encoded_secret_and_id))?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header);
    headers.insert("Content-Type", HeaderValue::from_static("applications/x-www-form-urlencoded"));

    let client = Client::new();
    let response = client
        .post(TOKEN_ENDPOINT)
        .headers(headers)
        .form(&params)
        .send()?;

    let body = response.text()?
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(SPOTIFY_AUTH_PATH)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
