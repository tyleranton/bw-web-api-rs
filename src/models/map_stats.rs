use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceStats {
    pub total_games: u32,
    pub total_global_games: u32,
    pub total_global_wins: u32,
    pub total_wins: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapRaceStats {
    #[serde(rename = "Protoss")]
    pub protoss: RaceStats,
    #[serde(rename = "Random")]
    pub random: RaceStats,
    #[serde(rename = "Terran")]
    pub terran: RaceStats,
    #[serde(rename = "Zerg")]
    pub zerg: RaceStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapStatsByToon {
    pub current_season: u32,
    pub map_stat: HashMap<String, HashMap<String, HashMap<String, MapRaceStats>>>,
}

impl MapStatsByToon {
    pub fn get_map_stats(
        &self,
        gamemode: &str,
        season: &str,
        map_hash: &str,
    ) -> Option<&MapRaceStats> {
        self.map_stat.get(gamemode)?.get(season)?.get(map_hash)
    }
}
