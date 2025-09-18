use crate::types::Gateway;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub last_rank: u32,
    pub gateway_id: Gateway,
    pub points: u32,
    pub wins: u32,
    pub losses: u32,
    pub disconnects: u32,
    pub toon: String,
    pub battletag: String,
    pub avatar: String,
    pub feature_stat: String,
    pub rating: u32,
    pub bucket: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardRankResponse {
    pub aurora_id: u32,
    pub gateway_id: u32,
    pub leaderboard_id: u32,
    pub matchmaked_current_season: u32,
    pub matchmaked_current_season_buckets: Vec<u32>,
    pub mingames: u32,
    pub toons: Vec<String>,
    pub total_rows: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMode {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayInfo {
    pub is_official: bool,
    pub name: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardInfo {
    pub benefactor_id: String,
    pub gamemode_id: u32,
    pub gateway_id: u32,
    pub id: u32,
    pub last_update_time: String,
    pub name: String,
    pub next_update_time: String,
    pub program_id: String,
    pub season_id: u32,
    pub season_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardMetadata {
    pub gamemodes: HashMap<String, GameMode>,
    pub gateways: HashMap<String, GatewayInfo>,
    pub leaderboards: HashMap<String, LeaderboardInfo>,
    pub matchmaked_current_season: u32,
    pub team_leaderboard_info: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

impl LeaderboardResponse {
    pub fn entries(&self) -> Vec<LeaderboardEntry> {
        self.rows
            .iter()
            .filter_map(|row| {
                if row.len() == 13 {
                    Some(LeaderboardEntry {
                        rank: row[0].as_u64()? as u32,
                        last_rank: row[1].as_u64()? as u32,
                        gateway_id: serde_json::from_value(row[2].clone()).ok()?,
                        points: row[3].as_u64()? as u32,
                        wins: row[4].as_u64()? as u32,
                        losses: row[5].as_u64()? as u32,
                        disconnects: row[6].as_u64()? as u32,
                        toon: row[7].as_str()?.to_string(),
                        battletag: row[8].as_str()?.to_string(),
                        avatar: row[9].as_str()?.to_string(),
                        feature_stat: row[10].as_str()?.to_string(),
                        rating: row[11].as_u64()? as u32,
                        bucket: row[12].as_u64()? as u32,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardNameSearchEntry {
    pub avatar: String,
    pub battletag: String,
    pub gateway_id: Gateway,
    pub last_rank: u32,
    pub name: String,
    pub points: u32,
    pub rank: u32,
}

pub type LeaderboardNameSearchResponse = Vec<LeaderboardNameSearchEntry>;
