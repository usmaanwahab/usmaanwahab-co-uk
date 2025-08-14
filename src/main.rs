#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::response::content::RawText;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};

use reqwest::Url;

mod spotify_utils;
use spotify_utils::{get_spotify_access_tokens, get_spotify_credentials};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
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
    match get_spotify_access_tokens(code) {
        Ok(tokens) => Ok("Success".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/spotify")]
fn spotify() -> Result<Redirect, String> {
    let spotify_credentials = get_spotify_credentials().map_err(|e| e.to_string())?;

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

// #[get("spotify/current")]
// fn current() -> Result<RawText<String>, String> {
//
// }

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![index, education, experience, projects, spotify, callback],
        )
        .attach(Template::fairing())
}
