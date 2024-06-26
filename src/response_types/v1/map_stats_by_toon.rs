use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MapStatsByToonResponse {
    NoResult(NoResultResponse),
    Result(MapStatResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoResultResponse {
    pub current_season: i32,
    pub map_stat: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapStatResponse {
    pub current_season: i32,
    pub map_stat: HashMap<String, HashMap<String, HashMap<String, RaceStats>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceStats {
    pub protoss: Stats,
    pub random: Stats,
    pub terran: Stats,
    pub zerg: Stats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub total_games: i32,
    pub total_global_games: i32,
    pub total_global_wins: i32,
    pub total_wins: i32,
}
