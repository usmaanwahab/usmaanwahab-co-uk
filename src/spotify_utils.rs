use crate::spotify_auth::authorised_spotify_client;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Track {
    pub name: String,
    pub image_url: String,
    pub artist: String,
}

pub fn get_current_track() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = "https://api.spotify.com/v1/me/player/currently-playing";
    let response = authorised_spotify_client(reqwest::Method::GET, url)?.send()?;

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
    let limit_str = limit.to_string();
    let offset_str = offset.to_string();

    let mut params = HashMap::new();
    params.insert("term", term);
    params.insert("limit", &limit_str);
    params.insert("offset", &offset_str);

    let url = format!("https://api.spotify.com/v1/me/top/{}", item_type);
    let response = authorised_spotify_client(reqwest::Method::GET, &url)?
        .query(&params)
        .send()?;
    // TODO is .send().error_for_status()?; worth? should save manual error handling
    if !response.status().is_success() {
        let body = response.text()?;
        return Err(Box::<dyn std::error::Error>::from(body));
    }

    let body = response.text()?;

    let json: serde_json::Value = serde_json::from_str(&body)?;

    Ok(json)
}

pub fn get_recently_played() -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let url = "https://api.spotify.com/v1/me/player/recently-played";
    let response = authorised_spotify_client(reqwest::Method::GET, url)?.send()?;
    // TODO is .send().error_for_status()?; worth?  should save manual error handling

    if !response.status().is_success() {
        let body = response.text()?;
        return Err(Box::<dyn std::error::Error>::from(body));
    }

    let body = response.text()?;
    let json: serde_json::Value = serde_json::from_str(&body)?;

    let tracks_json = json["items"].as_array().cloned().unwrap_or_default();
    let mut tracks: Vec<Track> = Vec::<Track>::new();

    for track in tracks_json {
        let track_info = Track {
            name: track["track"]["name"].as_str().unwrap_or("").to_string(),
            image_url: track["track"]["album"]["images"][2]["url"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            artist: track["track"]["artists"][0]["name"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        };
        tracks.push(track_info);
    }

    Ok(tracks)
}
