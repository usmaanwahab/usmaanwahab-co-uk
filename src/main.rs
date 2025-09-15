#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use reqwest::blocking::Client;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};

pub mod riot;
pub mod spotify;

use spotify::{
    callback, currently_playing_widget, spotify_authorise, spotify_homepage, spotify_recent,
    top_artists, top_tracks,
};

use riot::{LeagueV4, get_match_history, get_puuid_by_name_and_tag, get_ranked_stats_by_puuid};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/education")]
fn education() -> Template {
    let subjects: [&str; 14] = [
        "Advanced Systems Programming",
        "Cyber Security Fundamentals",
        "Software Engineering Release Practices",
        "Functional Programming",
        "Distributed and Parallel Technologies",
        "Computer Architecture",
        "Algorithmics 2",
        "Programming Languages",
        "Database Systems",
        "Networked Systems",
        "Operating Systems",
        "Algorithmics 1",
        "Data Fundamentals",
        "Systems Programming",
    ];
    Template::render("education", context! {subjects: subjects})
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
            Err(e) => format!("Failed to load GitHub file: {}", e),
        },
        Err(e) => format!("Request failed: {}", e),
    };
    Template::render("projects", context! {deploy_file_content: body})
}

#[get("/league")]
fn league() -> Result<Template, String> {
    let acc_v1 = get_puuid_by_name_and_tag("Weetabicx", "EUW").map_err(|e| e.to_string())?;
    let ranked_data = get_ranked_stats_by_puuid(&acc_v1.puuid).map_err(|e| e.to_string())?;

    let mut data: Option<&LeagueV4> = None;

    for entry in &ranked_data {
        if entry.queue_type == "RANKED_SOLO_5x5" {
            data = Some(&entry);
            break;
        }
    }

    let data = match data {
        Some(data) => data,
        None => return Err("Error - No data found".to_string()),
    };

    let total = data.wins + data.losses;
    let loss_percent = (data.losses as f64 / total as f64) * 100.0;
    let win_percent = (data.wins as f64 / total as f64) * 100.0;

    Ok(Template::render(
        "league",
        context! {data: data, win_percent: win_percent, loss_percent: loss_percent},
    ))
}

#[get("/league/match-history")]
fn match_history() -> Result<Template, String> {
    let acc_v1 = get_puuid_by_name_and_tag("Weetabicx", "EUW").map_err(|e| e.to_string())?;
    let match_history = get_match_history(&acc_v1.puuid).map_err(|e| e.to_string())?;

    Ok(Template::render(
        "league-match-history",
        context! {matches: match_history, puuid: acc_v1.puuid},
    ))
}
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/static", FileServer::from("/root/static"))
        .mount(
            "/",
            routes![
                index,
                education,
                experience,
                projects,
                league,
                match_history,
            ],
        )
        .mount(
            "/spotify",
            routes![
                spotify_authorise,
                callback,
                currently_playing_widget,
                top_tracks,
                top_artists,
                spotify_recent,
                spotify_homepage
            ],
        )
        .attach(Template::fairing())
}
