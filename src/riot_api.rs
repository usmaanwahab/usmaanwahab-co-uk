use serde_json;

use reqwest::{blocking::Client, header::HeaderMap, header::HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct RiotConfig {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountV1 {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueV4 {
    pub league_id: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub puuid: String,
    pub league_points: u16,
    pub wins: u16,
    pub losses: u16,
    pub veteran: bool,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub hot_streak: bool,
}

pub fn read_riot_api_key() -> Result<RiotConfig, Box<dyn std::error::Error>> {
    Ok(RiotConfig {
        key: env::var("RIOT_API_KEY").expect("RIOT_API_KEY not set"),
    })
}

pub fn get_puuid_by_name_and_tag(
    name: &str,
    tag: &str,
) -> Result<AccountV1, Box<dyn std::error::Error>> {
    let riot_config = read_riot_api_key()?;

    let url: String = format!(
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        name, tag
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&riot_config.key)?);

    let client = Client::new();
    let response = client.get(url).headers(headers).send()?;

    let body = response.text()?;
    let json: AccountV1 = serde_json::from_str(&body)?;

    Ok(json)
}

pub fn get_ranked_stats_by_puuid(
    puuid: &str,
) -> Result<Vec<LeagueV4>, Box<dyn ::std::error::Error>> {
    let riot_config = read_riot_api_key()?;

    let url: String = format!(
        "https://euw1.api.riotgames.com/lol/league/v4/entries/by-puuid/{}",
        puuid
    );

    let mut headers = HeaderMap::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(&riot_config.key)?);

    let client = Client::new();

    let response = client.get(url).headers(headers).send()?;
    let body = response.text()?;
    let json: Vec<LeagueV4> = serde_json::from_str(&body)?;

    Ok(json)
}
