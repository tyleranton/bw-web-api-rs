use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::Gateway;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub is_official: bool,
    pub name: String,
    pub online_users: u32,
    pub region: String,
}

pub type GatewayResponse = HashMap<Gateway, GatewayStatus>;