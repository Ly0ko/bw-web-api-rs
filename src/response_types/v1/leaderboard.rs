use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardResponse {
    pub gamemodes: HashMap<String, Gamemode>,
    pub gateways: HashMap<String, Gateway>,
    pub leaderboards: HashMap<String, Leaderboard>,
    pub matchmaked_current_season: i32,
    pub team_leaderboard_info: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gamemode {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gateway {
    pub is_official: bool,
    pub name: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaderboard {
    pub benefactor_id: String,
    pub gamemode_id: i32,
    pub gateway_id: i32,
    pub id: i32,
    pub last_update_time: String,
    pub name: String,
    pub next_update_time: String,
    pub program_id: String,
    pub season_id: i32,
    pub season_name: String,
}
