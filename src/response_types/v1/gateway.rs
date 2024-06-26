use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayInfo {
    pub is_official: bool,
    pub name: String,
    pub online_users: u32,
    pub region: String,
}

pub type GatewayResponse = HashMap<String, GatewayInfo>;
