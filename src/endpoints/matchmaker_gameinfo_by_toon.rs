use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::MatchmakerGameInfo;
use crate::types::Gateway;

pub struct MatchmakerGameInfoEndpoint {
    toon: String,
    gateway: Gateway,
    gamemode: u32,
    season: u32,
    offset: Option<u32>,
    limit: Option<u32>,
}

impl MatchmakerGameInfoEndpoint {
    pub fn new(
        toon: String,
        gateway: Gateway,
        gamemode: u32,
        season: u32,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            toon,
            gateway,
            gamemode,
            season,
            offset,
            limit,
        }
    }
}

impl Endpoint for MatchmakerGameInfoEndpoint {
    type Request = ();
    type Response = MatchmakerGameInfo;

    fn endpoint(&self) -> String {
        let mut url = format!(
            "/web-api/v1/matchmaker-gameinfo-by-toon/{}/{}/{}/{}",
            urlencoding::encode(&self.toon),
            self.gateway as i32,
            self.gamemode,
            self.season
        );

        if let Some(offset) = self.offset {
            url.push_str(&format!("?offset={}", offset));
            if let Some(limit) = self.limit {
                url.push_str(&format!("&limit={}", limit));
            }
        } else if let Some(limit) = self.limit {
            url.push_str(&format!("?limit={}", limit));
        }

        url
    }
}

impl crate::client::ApiClient {
    /// Get matchmaker game information for a specific player
    /// 
    /// Endpoint: /web-api/v1/matchmaker-gameinfo-by-toon/{toon}/{gateway}/{gamemode}/{season}
    pub async fn get_matchmaker_gameinfo(
        &self,
        toon: String,
        gateway: Gateway,
        gamemode: u32,
        season: u32,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<MatchmakerGameInfo, ApiError> {
        let endpoint = MatchmakerGameInfoEndpoint::new(toon, gateway, gamemode, season, offset, limit);
        self.request(&endpoint, &()).await
    }
}