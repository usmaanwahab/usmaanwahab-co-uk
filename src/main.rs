#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};

use reqwest::Url;

mod spotify_auth;
use spotify_auth::{
    request_spotify_access_token, 
    refresh_spotify_auth,
    read_spotify_credentials
};

mod spotify_utils;
use spotify_utils::{
    get_current_track,
};


#[get("/")]
fn index() -> Template {
    match refresh_spotify_auth() {
        Ok(()) => println!("refreshed token"),
        Err(e) => println!("Error: {}", e.to_string())
    };
    let current_track_data = match get_current_track() {
        Ok(body) => body,
        Err(e) => panic!{"{:?}", e}
    };
    let track_name = current_track_data["item"]["name"].as_str().unwrap_or("Nothing to see here...");
    let progress_ms = current_track_data["progress_ms"].as_i64().unwrap_or(-1);
    let duration_ms = current_track_data["item"]["duration_ms"].as_i64().unwrap_or(-1);
    let image_url = current_track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("");
    let artist_name = current_track_data["item"]["album"]["artists"][0]["name"].as_str().unwrap_or("");
    Template::render("index", context! {
        track_name: track_name,
        progress_ms: progress_ms,
        duration_ms: duration_ms,
        image_url: image_url,
        artist_name: artist_name,
    })
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
    Template::render("projects", context! {})
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
        ("scope", "user-read-currently-playing"),
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

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/static", FileServer::from("/root/static"))
        .mount("/", routes![index, education, experience, projects, spotify, callback])
        .attach(Template::fairing())
}
