use crate::spotify_auth::{read_spotify_auth, refresh_spotify_auth};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

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

#[derive(Serialize)]
pub struct TrackInfo {
    pub name: String,
    pub image_url: String,
    pub played_at: String,
    pub artist: String,
}

pub fn get_recently_played() -> Result<Vec<TrackInfo>, Box<dyn std::error::Error>> {
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
        .get("https://api.spotify.com/v1/me/player/recently-played")
        .headers(headers)
        .send()?;

    let body = response.text()?;
    let json: serde_json::Value = serde_json::from_str(&body)?;

    let tracks: &Vec<serde_json::Value> = json["items"].as_array().unwrap();
    let mut track_infos: Vec<TrackInfo> = Vec::<TrackInfo>::new();

    for track in tracks {
        let track_info = TrackInfo {
            name: track["track"]["name"].as_str().unwrap().to_string(),
            image_url: track["track"]["album"]["images"][2]["url"]
                .as_str()
                .unwrap()
                .to_string(),
            played_at: track["played_at"].as_str().unwrap().to_string(),
            artist: track["track"]["artists"][0]["name"]
                .as_str()
                .unwrap()
                .to_string(),
        };
        track_infos.push(track_info);
    }

    Ok(track_infos)
}
