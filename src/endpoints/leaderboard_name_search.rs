use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::LeaderboardNameSearchResponse;
use crate::types::Leaderboard;

pub struct LeaderboardNameSearchEndpoint {
    leaderboard_id: Leaderboard,
    battletag: String,
}

impl LeaderboardNameSearchEndpoint {
    pub fn new(leaderboard_id: Leaderboard, battletag: String) -> Self {
        Self {
            leaderboard_id,
            battletag,
        }
    }
}

impl Endpoint for LeaderboardNameSearchEndpoint {
    type Request = ();
    type Response = LeaderboardNameSearchResponse;

    fn endpoint(&self) -> String {
        format!(
            "/web-api/v1/leaderboard-name-search/{}/{}",
            self.leaderboard_id as i32,
            urlencoding::encode(&self.battletag)
        )
    }
}

impl crate::client::ApiClient {
    /// Search a leaderboard for players by battletag
    ///
    /// Endpoint: /web-api/v1/leaderboard-name-search/{leaderboard_id}/{battletag}
    pub async fn search_leaderboard_by_battletag(
        &self,
        leaderboard_id: Leaderboard,
        battletag: String,
    ) -> Result<LeaderboardNameSearchResponse, ApiError> {
        let endpoint = LeaderboardNameSearchEndpoint::new(leaderboard_id, battletag);
        self.request(&endpoint, &()).await
    }
}
