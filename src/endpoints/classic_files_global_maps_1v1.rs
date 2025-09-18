use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::classic_files_global_maps_1v1::GlobalMaps1v1;

pub struct GlobalMaps1v1Endpoint;

impl GlobalMaps1v1Endpoint {
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for GlobalMaps1v1Endpoint {
    type Request = ();
    type Response = GlobalMaps1v1;

    fn endpoint(&self) -> String {
        "/web-api/v1/file-set/classic.files.global.maps-1v1".to_string()
    }
}

impl crate::client::ApiClient {
    /// Get global 1v1 maps file set
    ///
    /// Endpoint: /web-api/v1/file-set/classic.files.global.maps-1v1
    pub async fn get_global_maps_1v1(&self) -> Result<GlobalMaps1v1, ApiError> {
        let endpoint = GlobalMaps1v1Endpoint::new();
        self.request(&endpoint, &()).await
    }
}
