use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapAttribute {
    pub map_candidate: String,
    pub map_description: String,
    pub map_era: String,
    pub map_height: String,
    pub map_md5: String,
    pub map_name: String,
    pub map_path: String,
    pub map_version: String,
    pub map_width: String,
    pub replay_humans: String,
    pub replay_max_players: String,
    pub replay_min_players: String,
    pub replay_opponents: String,
    pub season_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapInfo {
    pub attribute: MapAttribute,
    pub content_size: u32,
    pub content_type: String,
    pub md5: String,
    pub modified_epoch: u64,
    pub name: String,
    pub url: String,
}

pub type GlobalMaps1v1 = Vec<MapInfo>;
