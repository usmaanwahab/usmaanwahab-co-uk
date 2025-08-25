#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};
use rocket::response::content::RawHtml;
use reqwest::Url;
use reqwest::blocking::Client;

mod spotify_auth;
use spotify_auth::{
    request_spotify_access_token, 
    refresh_spotify_auth,
    read_spotify_credentials, 
};

mod spotify_utils;
use spotify_utils::{
    get_current_track,
    get_top_items
};


#[get("/")]
fn index() -> Template {
       Template::render("index", context!{})
}

#[get("/education")]
fn education() -> Template {
    Template::render("education", context! {})
}

#[get("/experience")]
fn experience() -> Template {
    Template::render("experience", context! {})
}

#[get("/projects")]
fn projects() -> Template {
    let client = Client::new();
    let response = client
        .get("https://raw.githubusercontent.com/usmaanwahab/usmaanwahab-co-uk/refs/heads/main/deploy.sh")
        .send();
    let body = match response {
        Ok(r) => match r.text() {
            Ok(text) => text,
            Err(e) => format!("Failed to load GitHub file: {}", e)
        },
        Err(e) => format!("Request failed: {}", e) 
    };
    Template::render("projects", context! {deploy_file_content: body})
}

#[get("/callback?<code>")]
fn callback(code: &str) -> Result<String, String> {
    match request_spotify_access_token(code) {
        Ok(()) => Ok("Success".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/spotify")]
fn spotify() -> Result<Redirect, String> {
    let spotify_credentials = match read_spotify_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("Failed to read spotify response: {}", e);
            return Err(e.to_string());
        }
    };

    let params = [
        ("response_type", "code"),
        ("client_id", spotify_credentials.client_id.as_str()),
        ("scope", "user-read-currently-playing user-top-read"),
        ("redirect_uri", "https://usmaanwahab.co.uk/callback"),
    ];

    let url = Url::parse_with_params("https://accounts.spotify.com/authorize", &params)
        .unwrap()
        .to_string();
    Ok(Redirect::to(url))
}

#[get("/spotify/refresh")]
fn refresh() -> Result<String, String> {
    match refresh_spotify_auth() {
        Ok(()) => Ok("Success".to_string()),
        Err(e) => Err(e.to_string())
    }
}

#[get("/spotify/currently-playing")]
fn currently_playing_widget() -> Result<Template, RawHtml<String>> {
    match refresh_spotify_auth() {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e.to_string())
    };
    let current_track_data = match get_current_track() {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Fetching current track failed: {}", e);
            return Err(RawHtml("Fetching current track failed".to_string()));
        }
    };

    let track_name = current_track_data["item"]["name"].as_str().unwrap_or("Nothing playing...");
    let progress_ms = current_track_data["progress_ms"].as_i64().unwrap_or(-1);
    let duration_ms = current_track_data["item"]["duration_ms"].as_i64().unwrap_or(-1);
    let image_url = current_track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("/static/pause.jpg");
    let artist_name = current_track_data["item"]["album"]["artists"][0]["name"].as_str().unwrap_or("");

    Ok(Template::render("audio-player", context! {
        track_name: track_name,
        progress_ms: progress_ms,
        duration_ms: duration_ms,
        image_url: image_url,
        artist_name: artist_name,
    }))
}



#[get("/spotify/top/tracks/<term>?<limit>&<offset>")]
fn top_tracks(term: String, limit: Option<u16>, offset: Option<u16>) -> Result<Template, RawHtml<String>> {
    let term = match term.as_str() {
        "short_term" => term,
        "medium_term" => term,
        "long_term" => term,
        _ => return Err(RawHtml("Term is not valid.".to_string()))
    };
    
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let top_tracks = match get_top_items("tracks", &term, limit, offset) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error - could not fetch top tracks: {}", e);
            return Err(RawHtml("Error - could not fetch top tracks".to_string()));
        } 
    };
    
    Ok(Template::render("top-tracks", context!{
        data: top_tracks
    }))
}

#[get("/spotify/top/artists/<term>?<limit>&<offset>")]
fn top_artists(term: String, limit: Option<u16>, offset: Option<u16> ) -> Result<Template, RawHtml<String>> {
    let term = match term.as_str() {
        "short_term" => term,
        "medium_term" => term,
        "long_term" => term,
        _ => return Err(RawHtml("Term is not valid.".to_string()))
    };
    
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let top_tracks = match get_top_items("artists", &term, limit, offset) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error - could not fetch top tracks: {}", e);
            return Err(RawHtml("Error - could not fetch top tracks".to_string()));
        } 
    };
    
    Ok(Template::render("top-tracks", context!{
        data: top_tracks
    }))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/static", FileServer::from("/root/static"))
        .mount("/", routes![index, education, experience, projects, spotify, callback, currently_playing_widget, top_artists, top_tracks])
        .attach(Template::fairing())
}
