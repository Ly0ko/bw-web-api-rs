use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardRankByToonResponse {
    pub aurora_id: Option<i32>,
    pub gateway_id: Option<i32>,
    pub leaderboard_id: i32,
    pub matchmaked_current_season: i32,
    pub matchmaked_current_season_buckets: Vec<i32>,
    pub mingames: Option<i32>,
    pub total_rows: Option<i32>,
    pub toons: Vec<Toon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoResultResponse {
    pub leaderboard_id: i32,
    pub matchmaked_current_season: i32,
    pub matchmaked_current_season_buckets: Vec<i32>,
    pub toons: Vec<Toon>, // This should be an empty array
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Toon {
    pub avatar: String,
    pub battletag: String,
    pub bucket: i32,
    pub disconnects: i32,
    pub feature_stat: String,
    pub gateway_id: i32,
    pub last_rank: i32,
    pub losses: i32,
    pub name: String,
    pub points: i32,
    pub rank: i32,
    pub wins: i32,
}
