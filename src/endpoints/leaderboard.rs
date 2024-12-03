use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::LeaderboardMetadata;

pub struct Leaderboard;

impl Leaderboard {
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for Leaderboard {
    type Request = ();
    type Response = LeaderboardMetadata;

    fn endpoint(&self) -> String {
        "/web-api/v1/leaderboard".to_string()
    }
}

impl crate::client::ApiClient {
    /// Get leaderboard metadata including available gamemodes, gateways, and leaderboards
    /// 
    /// Endpoint: /web-api/v1/leaderboard
    pub async fn get_leaderboard(&self) -> Result<LeaderboardMetadata, ApiError> {
        let endpoint = Leaderboard::new();
        self.request(&endpoint, &()).await
    }
}