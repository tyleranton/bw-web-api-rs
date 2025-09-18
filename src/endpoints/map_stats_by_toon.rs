use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::MapStatsByToon;
use crate::types::Gateway;

pub struct MapStatsByToonEndpoint {
    toon: String,
    gateway: Gateway,
}

impl MapStatsByToonEndpoint {
    pub fn new(toon: String, gateway: Gateway) -> Self {
        Self { toon, gateway }
    }
}

impl Endpoint for MapStatsByToonEndpoint {
    type Request = ();
    type Response = MapStatsByToon;

    fn endpoint(&self) -> String {
        format!(
            "/web-api/v1/map-stats-by-toon/{}/{}",
            urlencoding::encode(&self.toon),
            self.gateway as i32
        )
    }
}

impl crate::client::ApiClient {
    /// Get map statistics for a specific player
    ///
    /// Endpoint: /web-api/v1/map-stats-by-toon/{toon}/{gateway}
    pub async fn get_map_stats(
        &self,
        toon: String,
        gateway: Gateway,
    ) -> Result<MapStatsByToon, ApiError> {
        let endpoint = MapStatsByToonEndpoint::new(toon, gateway);
        self.request(&endpoint, &()).await
    }
}
