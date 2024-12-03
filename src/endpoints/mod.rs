pub mod aurora_profile_by_toon;
pub mod classic_files_global_maps_1v1;
pub mod gateway;
pub mod leaderboard_entity;
pub mod leaderboard_rank_by_toon;
pub mod leaderboard;
pub mod map_stats_by_toon;
pub mod matchmaker_gameinfo_by_toon;
pub mod matchmaker_gameinfo_playerinfo;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::ApiError;

#[async_trait]
pub trait Endpoint {
    type Request: Serialize;
    type Response: for<'de> Deserialize<'de>;

    fn endpoint(&self) -> String;

    fn validate_response(_response: &Self::Response) -> Result<(), ApiError> {
        Ok(())
    }
}
