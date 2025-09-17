use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrMmGameLoading {
    pub aurora_id: u32,
    pub battle_tag: String,
    pub country_code: String,
    pub matchmaked_current_season: u32,
    pub matchmaked_current_season_buckets: Vec<u32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub program_id: String,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, u32>>,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct ScrMmToonInfo {
    pub aurora_id: u32,
    pub battle_tag: String,
    pub country_code: String,
    pub matchmaked_current_season: u32,
    pub matchmaked_current_season_buckets: Vec<u32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub program_id: String,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, u32>>,
    pub toons: Vec<Toon>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrProfile {
    pub aurora_id: u32,
    pub avatars: HashMap<String, String>,
    pub avatars_framed: HashMap<String, Avatar>,
    #[serde(default)]
    pub avatars_locked: HashMap<String, Avatar>,
    #[serde(default)]
    pub avatars_locked_framed: HashMap<String, Avatar>,
    pub avatars_unlocked: HashMap<String, Avatar>,
    #[serde(default)]
    pub avatars_stats: Option<AvatarsStats>,
    #[serde(default)]
    pub battle_tag: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub game_results: Vec<GameResult>,
    pub matchmaked_current_season: u32,
    pub matchmaked_current_season_buckets: Vec<u32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub profiles: Option<Vec<Profile>>,
    pub program_id: String,
    pub replays: Vec<Replay>,
    pub stats: Vec<Stat>,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, u32>>,
    pub toons: Vec<Toon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrToonInfo {
    pub aurora_id: u32,
    pub avatars: HashMap<String, String>,
    pub avatars_framed: HashMap<String, Avatar>,
    #[serde(default)]
    pub avatars_locked: HashMap<String, Avatar>,
    #[serde(default)]
    pub avatars_locked_framed: HashMap<String, Avatar>,
    pub avatars_unlocked: HashMap<String, Avatar>,
    pub battle_tag: String,
    pub country_code: String,
    pub matchmaked_current_season: u32,
    pub matchmaked_current_season_buckets: Vec<u32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub profiles: Option<Vec<Profile>>,
    pub program_id: String,
    pub stats: Vec<Stat>,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, u32>>,
    pub toons: Vec<Toon>,
}
