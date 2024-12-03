use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};

#[repr(i32)]
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gateway {
    USWest = 10,
    USEast = 11,
    Europe = 20,
    Korea = 30,
    Asia = 45,
}

impl<'de> Deserialize<'de> for Gateway {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum GatewayId {
            Num(i32),
            Str(String),
        }

        let gateway_id = GatewayId::deserialize(deserializer)?;
        
        let id = match gateway_id {
            GatewayId::Num(n) => n,
            GatewayId::Str(s) => i32::from_str(&s).map_err(serde::de::Error::custom)?,
        };

        match id {
            10 => Ok(Gateway::USWest),
            11 => Ok(Gateway::USEast),
            20 => Ok(Gateway::Europe),
            30 => Ok(Gateway::Korea),
            45 => Ok(Gateway::Asia),
            other => Err(serde::de::Error::custom(format!(
                "Unknown gateway id: {}", 
                other
            ))),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Leaderboard {
    Global = 12960,
    USWest = 12961,
    USEast = 12962,
    Europe = 12963,
    Korea = 12964,
    Asia = 12965,
}