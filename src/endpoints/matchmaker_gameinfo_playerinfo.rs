use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::MatchmakerPlayerInfo;

pub struct MatchmakerPlayerInfoEndpoint {
    match_id: String,
}

impl MatchmakerPlayerInfoEndpoint {
    pub fn new(match_id: String) -> Self {
        Self { match_id }
    }
}

impl Endpoint for MatchmakerPlayerInfoEndpoint {
    type Request = ();
    type Response = MatchmakerPlayerInfo;

    fn endpoint(&self) -> String {
        format!(
            "/web-api/v1/matchmaker-gameinfo-playerinfo/{}",
            urlencoding::encode(&self.match_id)
        )
    }
}

impl crate::client::ApiClient {
    /// Get detailed player information for a specific match
    ///
    /// Endpoint: /web-api/v1/matchmaker-gameinfo-playerinfo/{match_id}
    pub async fn get_matchmaker_player_info(
        &self,
        match_id: String,
    ) -> Result<MatchmakerPlayerInfo, ApiError> {
        let endpoint = MatchmakerPlayerInfoEndpoint::new(match_id);
        self.request(&endpoint, &()).await
    }
}
