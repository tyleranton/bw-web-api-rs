use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::LeaderboardRankResponse;
use crate::types::{Gateway, Leaderboard};

pub struct LeaderboardRankEndpoint {
    leaderboard_id: Leaderboard,
    toon: String,
    gateway: Gateway,
}

impl LeaderboardRankEndpoint {
    pub fn new(leaderboard_id: Leaderboard, toon: String, gateway: Gateway) -> Self {
        Self {
            leaderboard_id,
            toon,
            gateway,
        }
    }
}

impl Endpoint for LeaderboardRankEndpoint {
    type Request = ();
    type Response = LeaderboardRankResponse;

    fn endpoint(&self) -> String {
        format!(
            "/web-api/v1/leaderboard-rank-by-toon/{}/{}/{}",
            self.leaderboard_id as i32,
            urlencoding::encode(&self.toon),
            self.gateway as i32
        )
    }
}

impl crate::client::ApiClient {
    /// Get leaderboard rank for a specific player
    ///
    /// Endpoint: /web-api/v1/leaderboard-rank-by-toon/{leaderboard_id}/{toon}/{gateway}
    pub async fn get_leaderboard_rank(
        &self,
        leaderboard_id: Leaderboard,
        toon: String,
        gateway: Gateway,
    ) -> Result<LeaderboardRankResponse, ApiError> {
        let endpoint = LeaderboardRankEndpoint::new(leaderboard_id, toon, gateway);
        self.request(&endpoint, &()).await
    }
}
