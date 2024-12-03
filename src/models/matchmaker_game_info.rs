use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::Gateway;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameAttributes {
    pub closed_slots: String,
    pub flags: String,
    pub game_speed: String,
    pub host_name: String,
    pub is_replay: String,
    pub map_crc: String,
    pub map_file_name: String,
    pub map_file_size: String,
    pub map_height: String,
    pub map_md5: String,
    pub map_name: String,
    pub map_tile_set: String,
    pub map_width: String,
    pub net_turn_rate: String,
    pub observers_current: String,
    pub observers_max: String,
    pub players_ai: String,
    pub players_current: String,
    pub players_max: String,
    pub proxy: String,
    pub rank: String,
    pub save_game_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub attributes: GameAttributes,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerResult {
    pub attributes: PlayerResultAttributes,
    pub is_computer: bool,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerResultAttributes {
    #[serde(rename = "gPlayerData_idx")]
    pub player_data_idx: String,
    pub left: String,
    #[serde(default)]
    pub race: Option<String>,
    #[serde(default)]
    pub team: Option<String>,
    #[serde(rename = "type")]
    pub player_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerGameResult {
    #[serde(rename = "")]
    pub empty: PlayerResult,
    #[serde(flatten)]
    pub players: HashMap<String, PlayerResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfoAttributes {
    #[serde(rename = "_default_region", default)]
    pub default_region: Option<String>,
    #[serde(rename = "_email", default)]
    pub email: Option<String>,
    pub connection_info: String,
    pub map: String,
    #[serde(default)]
    pub map_selection: Option<String>,
    pub player_battle_tag: String,
    pub player_legacy_gateway_id: String,
    pub player_legacy_toon_name: String,
    pub player_region: String,
    pub player_routing_via_proxy_server: String,
    pub race: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerScore {
    pub base: i32,
    pub bucket_new: i32,
    pub bucket_old: i32,
    pub delta: i32,
    pub win_streak: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub aurora_id: u64,
    pub avatar_url: String,
    pub benefactor_id: String,
    pub game_info: GameInfo,
    pub game_result: MatchmakerGameResult,
    pub gateway_id: Gateway,
    pub info_attributes: PlayerInfoAttributes,
    pub is_winner: String,
    pub matching_attributes: HashMap<String, String>,
    pub name: String,
    pub score: PlayerScore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub match_created: String,
    pub players: Vec<HashMap<String, PlayerInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerGameInfo(pub Vec<HashMap<String, Match>>);