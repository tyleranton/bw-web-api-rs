use super::common::{Avatar, ReplayAttributes, WinStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerPlayerStats {
    pub game_mode_id: u32,
    pub losses: u32,
    pub min_games: u32,
    pub season_id: u32,
    pub total_games: u32,
    pub wins: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerPlayerScore {
    pub base: i32,
    pub bucket_new: i32,
    pub bucket_old: i32,
    pub current_stat_bucket: i32,
    pub current_stat_losses: i32,
    pub current_stat_wins: i32,
    pub delta: i32,
    pub season_id: i32,
    pub win_streak: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerReplay {
    pub attributes: ReplayAttributes,
    pub create_time: u64,
    pub link: String,
    pub md5: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerPlayerInfoPlayer {
    pub aurora_id: u64,
    pub benefactor_id: String,
    pub game_info: super::matchmaker_game_info::GameInfo,
    pub game_result: super::matchmaker_game_info::MatchmakerGameResult,
    pub gateway_id: crate::types::Gateway,
    pub info_attributes: super::matchmaker_game_info::PlayerInfoAttributes,
    pub matching_attributes: HashMap<String, String>,
    pub name: String,
    pub score: MatchmakerPlayerScore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakerPlayerInfo {
    pub avatars: HashMap<String, String>,
    pub avatars_awards: HashMap<String, f64>,
    pub avatars_locked: HashMap<String, Avatar>,
    pub avatars_stats: HashMap<String, WinStats>,
    pub maps: Vec<String>,
    pub matchmaked_season_buckets: HashMap<String, Vec<i32>>,
    pub player_stats: Vec<MatchmakerPlayerStats>,
    pub players: HashMap<String, MatchmakerPlayerInfoPlayer>,
    pub replays: Vec<MatchmakerReplay>,
}
