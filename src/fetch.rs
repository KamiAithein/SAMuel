use serde::ser::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result as SerdeResult, Value as SerdeValue};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, write};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs, path::Path};
use steamworks::{AppId, Client};

pub fn print_header() {
    println!("{:<10}{:<50}{:<10}", "id", "name", "playtime (h)");
}

#[derive(Debug, Deserialize)]
pub struct SteamOwnedGames {
    pub response: SteamResponse,
}

#[derive(Debug, Deserialize)]
pub struct SteamResponse {
    pub game_count: usize,
    pub games: Vec<SteamGame>,
}

#[derive(Debug, Deserialize)]
pub struct SteamGame {
    pub appid: usize,
    pub name: String,
    pub playtime_forever: usize,
}

impl Display for SteamGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10}{:<50}{:<10.1}",
            self.appid,
            (&self.name[..47.min(self.name.len())]).to_string()
                + if self.name.len() >= 47 { "..." } else { "" },
            self.playtime_forever as f32 / 60.0
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum SteamFetchError {
    DeserializeError(String),
    ReadCacheError,
    FetchError(String),
    GetTextError,
    WriteCacheError(String),
}

impl Display for SteamFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SteamFetchError::DeserializeError(e) => writeln!(f, "Deserialize Error!\n{e}"),
            SteamFetchError::ReadCacheError => writeln!(f, "Read Cache Error!"),
            SteamFetchError::WriteCacheError(e) => writeln!(f, "Write Cache Error!\n{e}"),
            SteamFetchError::FetchError(e) => writeln!(f, "Fetch Error!\n{e}"),
            SteamFetchError::GetTextError => writeln!(f, "Get Text Error!"),
        }
    }
}

impl From<reqwest::Error> for SteamFetchError {
    fn from(value: reqwest::Error) -> Self {
        SteamFetchError::FetchError("TODO!".to_owned())
    }
}

impl std::error::Error for SteamFetchError {}

pub async fn get_owned_games_direct(
    api_key: &str,
    steam_id: &str,
    cache_path: &PathBuf,
) -> Result<SteamOwnedGames, SteamFetchError> {
    let response = reqwest::get(format!(
            "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?key={api_key}&steamid={steam_id}&include_appinfo=true&format=json"
        )).await
        .map_err(|_|SteamFetchError::FetchError("TODO".to_owned()))?
        .text() //TODO handle 3xx4xx5xx
        .await
        .map_err(|_|SteamFetchError::GetTextError)?;

    let mut cache =
        File::create(cache_path).map_err(|e| SteamFetchError::WriteCacheError(e.to_string()))?;
    cache
        .write_all(response.as_bytes())
        .map_err(|e| SteamFetchError::WriteCacheError(e.to_string()))?;

    let steam_owned_games: SteamOwnedGames =
        serde_json::from_str(&response).expect("Could not deserialize");

    Ok(steam_owned_games)
}

pub async fn get_owned_games(
    api_key: &str,
    steam_id: &str,
    cache_path: &PathBuf,
) -> Result<SteamOwnedGames, SteamFetchError> {
    let steam_owned_games = if cache_path.exists() {
        let cached_response =
            fs::read_to_string(cache_path).map_err(|e| SteamFetchError::ReadCacheError)?;
        let steam_owned_games: SteamOwnedGames =
            serde_json::from_str(&cached_response).expect("Could not deserialize");

        steam_owned_games
    } else {
        let steam_owned_games = get_owned_games_direct(api_key, steam_id, &cache_path).await?;

        steam_owned_games
    };

    Ok(steam_owned_games)
}
