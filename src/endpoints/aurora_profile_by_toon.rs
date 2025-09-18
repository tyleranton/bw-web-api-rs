use serde::Serialize;

use crate::endpoints::Endpoint;
use crate::error::ApiError;
use crate::models::{ScrMmGameLoading, ScrMmToonInfo, ScrProfile, ScrToonInfo};
use crate::types::Gateway;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum AuroraProfileFieldMask {
    #[serde(rename = "scr_profile")]
    ScrProfile,
    #[serde(rename = "scr_mmgameloading")]
    ScrMmGameLoading,
    #[serde(rename = "scr_mmtooninfo")]
    ScrMmToonInfo,
    #[serde(rename = "scr_tooninfo")]
    ScrToonInfo,
}

pub struct AuroraProfileEndpoint {
    toon: String,
    gateway: Gateway,
    field_mask: AuroraProfileFieldMask,
}

impl AuroraProfileEndpoint {
    pub fn new(toon: String, gateway: Gateway, field_mask: AuroraProfileFieldMask) -> Self {
        Self {
            toon,
            gateway,
            field_mask,
        }
    }
}

impl Endpoint for AuroraProfileEndpoint {
    type Request = ();
    type Response = serde_json::Value;

    fn endpoint(&self) -> String {
        format!(
            "/web-api/v2/aurora-profile-by-toon/{}/{}?request_flags={}",
            urlencoding::encode(&self.toon),
            self.gateway as i32,
            match self.field_mask {
                AuroraProfileFieldMask::ScrProfile => "scr_profile",
                AuroraProfileFieldMask::ScrMmGameLoading => "scr_mmgameloading",
                AuroraProfileFieldMask::ScrMmToonInfo => "scr_mmtooninfo",
                AuroraProfileFieldMask::ScrToonInfo => "scr_tooninfo",
            }
        )
    }

    fn validate_response(response: &serde_json::Value) -> Result<(), ApiError> {
        if let Some(aurora_id) = response.get("aurora_id") {
            if aurora_id == 0 {
                return Err(ApiError::ValidationError("Profile not found".to_string()));
            }
        }
        Ok(())
    }
}

impl crate::client::ApiClient {
    /// Get aurora profile with scr_profile flag
    ///
    /// Endpoint: /web-api/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_profile
    pub async fn get_aurora_profile_by_toon_scr_profile(
        &self,
        toon: String,
        gateway: Gateway,
    ) -> Result<ScrProfile, ApiError> {
        let endpoint =
            AuroraProfileEndpoint::new(toon, gateway, AuroraProfileFieldMask::ScrProfile);
        let json = self.request(&endpoint, &()).await?;

        AuroraProfileEndpoint::validate_response(&json)?;
        serde_json::from_value(json).map_err(Into::into)
    }

    /// Get aurora profile with scr_mmgameloading flag
    ///
    /// Endpoint: /web-api/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_mmgameloading
    pub async fn get_aurora_profile_by_toon_mm_game_loading(
        &self,
        toon: String,
        gateway: Gateway,
    ) -> Result<ScrMmGameLoading, ApiError> {
        let endpoint =
            AuroraProfileEndpoint::new(toon, gateway, AuroraProfileFieldMask::ScrMmGameLoading);
        let json = self.request(&endpoint, &()).await?;

        AuroraProfileEndpoint::validate_response(&json)?;
        serde_json::from_value(json).map_err(Into::into)
    }

    /// Get aurora profile with scr_mmtooninfo flag
    ///
    /// Endpoint: /web-api/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_mmtooninfo
    pub async fn get_aurora_profile_by_toon_mm_toon_info(
        &self,
        toon: String,
        gateway: Gateway,
    ) -> Result<ScrMmToonInfo, ApiError> {
        let endpoint =
            AuroraProfileEndpoint::new(toon, gateway, AuroraProfileFieldMask::ScrMmToonInfo);
        let json = self.request(&endpoint, &()).await?;

        AuroraProfileEndpoint::validate_response(&json)?;
        serde_json::from_value(json).map_err(Into::into)
    }

    /// Get aurora profile with scr_tooninfo flag
    ///
    /// Endpoint: /web-api/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_tooninfo
    pub async fn get_aurora_profile_by_toon_toon_info(
        &self,
        toon: String,
        gateway: Gateway,
    ) -> Result<ScrToonInfo, ApiError> {
        let endpoint =
            AuroraProfileEndpoint::new(toon, gateway, AuroraProfileFieldMask::ScrToonInfo);
        let json = self.request(&endpoint, &()).await?;

        AuroraProfileEndpoint::validate_response(&json)?;
        serde_json::from_value(json).map_err(Into::into)
    }
}
