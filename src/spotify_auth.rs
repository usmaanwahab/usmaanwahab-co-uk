use std::collections::HashMap;

use reqwest::RequestBuilder;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, SystemTime};

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use base64::Engine;
use base64::engine::general_purpose;

const SPOTIFY_AUTH_PATH: &str = "spotify_auth.json";
const REDIRECT_URI: &str = "https://usmaanwahab.co.uk/callback";
const TOKEN_ENDPOINT: &str = "https://accounts.spotify.com/api/token";

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyAuthCredentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: Option<String>,
    pub expires_in: u16,
    pub refresh_token: Option<String>,
}

pub fn read_spotify_credentials() -> Result<SpotifyAuthCredentials, Box<dyn std::error::Error>> {
    let spotify_credentials = SpotifyAuthCredentials {
        client_id: env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set"),
        client_secret: env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set"),
    };
    Ok(spotify_credentials)
}

pub fn read_spotify_auth() -> Result<SpotifyAuthResponse, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(SPOTIFY_AUTH_PATH)?;
    let spotify_auth_response: SpotifyAuthResponse = serde_json::from_str(&json_data)?;
    Ok(spotify_auth_response)
}

pub fn request_spotify_access_token(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let spotify_credentials = match read_spotify_credentials() {
        Ok(credentials) => credentials,
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

    let auth_header = HeaderValue::from_str(&format!("Basic {}", encoded_secret_and_id))?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header);
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("applications/x-www-form-urlencoded"),
    );

    let client = Client::new();
    let response = client
        .post(TOKEN_ENDPOINT)
        .headers(headers)
        .form(&params)
        .send()?;

    let body = response.text()?;
    let json_data: SpotifyAuthResponse = serde_json::from_str(&body)?;
    let formatted_json = serde_json::to_string_pretty(&json_data)?;
    let mut file = File::create(SPOTIFY_AUTH_PATH)?;
    file.write_all(formatted_json.as_bytes())?;

    Ok(())
}

pub fn refresh_spotify_auth() -> Result<(), Box<dyn std::error::Error>> {
    let spotify_credentials = match read_spotify_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("Failed to read spotify response: {}", e);
            return Err(e);
        }
    };

    let spotify_auth_response = match read_spotify_auth() {
        Ok(spotify_auth) => spotify_auth,
        Err(e) => {
            eprintln!("Failed to read spotify auth data: {}", e);
            return Err(e);
        }
    };

    // Don't refresh is token is still valid
    let metadata = fs::metadata(SPOTIFY_AUTH_PATH)?;
    let last_refresh = metadata.modified()?;
    let expiry_time = last_refresh + Duration::from_secs(spotify_auth_response.expires_in as u64);
    let now = SystemTime::now();

    if now < expiry_time {
        println!("No need to refresh token!");
        return Ok(());
    }

    let refresh_token = match spotify_auth_response.refresh_token {
        Some(t) => t,
        _ => panic!("Could not read old refresh_token - This should not be possible."),
    };

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &refresh_token);

    let encoded_secret_and_id = general_purpose::STANDARD.encode(&format!(
        "{}:{}",
        spotify_credentials.client_id, spotify_credentials.client_secret
    ));

    let auth_header = HeaderValue::from_str(&format!("Basic {}", encoded_secret_and_id))?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header);
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("applications/x-www-form-urlencoded"),
    );

    let client = Client::new();
    let response = client
        .post(TOKEN_ENDPOINT)
        .headers(headers)
        .form(&params)
        .send()?;

    let body = response.text()?;
    let mut json: SpotifyAuthResponse = serde_json::from_str(&body)?;
    json.refresh_token = Some(refresh_token);

    let json_str = serde_json::to_string_pretty(&json)?;
    let mut file = File::create(SPOTIFY_AUTH_PATH)?;
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn authorised_spotify_client(
    method: reqwest::Method,
    url: &str,
) -> Result<blocking::RequestBuilder, Box<dyn std::error::Error>> {
    refresh_spotify_auth()?;
    let spotify_auth_response = read_spotify_auth()?;

    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", &spotify_auth_response.access_token);
    headers.insert("Authorization", HeaderValue::from_str(&bearer_token)?);

    Ok(Client::new().request(method, url).headers(headers))
}
