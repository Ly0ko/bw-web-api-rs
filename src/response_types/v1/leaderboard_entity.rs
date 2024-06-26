use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardEntityResponse {
    pub columns: [String; 13],
    pub rows: Vec<LeaderboardRow>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardRow {
    pub rank: i32,
    pub last_rank: i32,
    pub gateway_id: i32,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    pub disconnects: i32,
    pub toon: String,
    pub battletag: String,
    pub avatar: String,
    pub feature_stat: String,
    pub rating: i32,
    pub bucket: i32,
}
