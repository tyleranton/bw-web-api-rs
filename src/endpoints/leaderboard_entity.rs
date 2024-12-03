use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::LeaderboardResponse;
use crate::types::Leaderboard;

pub struct LeaderboardEntityEndpoint {
    leaderboard_id: Leaderboard,
}

impl LeaderboardEntityEndpoint {
    pub fn new(leaderboard_id: Leaderboard) -> Self {
        Self { leaderboard_id }
    }
}

impl Endpoint for LeaderboardEntityEndpoint {
    type Request = ();
    type Response = LeaderboardResponse;

    fn endpoint(&self) -> String {
        format!("/web-api/v1/leaderboard/{}", self.leaderboard_id as i32)
    }
}

impl crate::client::ApiClient {
    /// Get leaderboard entries
    /// 
    /// Endpoint: /web-api/v1/leaderboard/{leaderboard_id}
    pub async fn get_leaderboard_entity(
        &self,
        leaderboard_id: Leaderboard,
    ) -> Result<LeaderboardResponse, ApiError> {
        let endpoint = LeaderboardEntityEndpoint::new(leaderboard_id);
        self.request(&endpoint, &()).await
    }
}