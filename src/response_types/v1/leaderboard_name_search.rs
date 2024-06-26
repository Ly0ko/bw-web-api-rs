use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    avatar: String,
    battletag: String,
    gateway_id: i32,
    last_rank: i32,
    name: String,
    points: i32,
    rank: i32,
}

pub type LeaderboardNameSearchResponse = Vec<LeaderboardEntry>;
