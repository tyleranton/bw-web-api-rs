use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::GatewayResponse;

pub struct GatewayEndpoint;

impl GatewayEndpoint {
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for GatewayEndpoint {
    type Request = ();
    type Response = GatewayResponse;

    fn endpoint(&self) -> String {
        "/web-api/v1/gateway".to_string()
    }
}

impl crate::client::ApiClient {
    /// Get status of all gateways
    ///
    /// Endpoint: /web-api/v1/gateway
    pub async fn get_gateway_status(&self) -> Result<GatewayResponse, ApiError> {
        let endpoint = GatewayEndpoint::new();
        self.request(&endpoint, &()).await
    }
}
