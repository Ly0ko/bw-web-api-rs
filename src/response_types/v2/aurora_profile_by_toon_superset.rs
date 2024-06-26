use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::common::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuroraProfileByToonSupersetResponse {
    NoResult(AuroraProfileByToonSupersetNoResultResponse),
    Result(AuroraProfileByToonSupersetResultResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuroraProfileByToonSupersetNoResultResponse {
    pub aurora_id: i32,
    pub avatars: HashMap<String, String>,
    pub avatars_framed: HashMap<String, AvatarFramed>,
    pub avatars_unlocked: HashMap<String, AvatarUnlocked>,
    pub game_results: Vec<GameResult>,
    pub matchmaked_current_season: i32,
    pub matchmaked_current_season_buckets: Vec<i32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub profiles: Option<Vec<Profile>>,
    pub program_id: String,
    pub replays: Vec<Replay>,
    pub stats: Vec<Stat>,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, i32>>,
    pub toons: Vec<Toon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuroraProfileByToonSupersetResultResponse {
    pub account_flags: Option<String>,
    pub aurora_id: i32,
    pub avatars: HashMap<String, String>,
    pub avatars_framed: HashMap<String, AvatarFramed>,
    pub avatars_unlocked: HashMap<String, AvatarUnlocked>,
    pub battle_tag: String,
    pub country_code: String,
    pub game_results: Vec<GameResult>,
    pub matchmaked_current_season: i32,
    pub matchmaked_current_season_buckets: Vec<i32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub profiles: Option<Vec<Profile>>,
    pub program_id: String,
    pub replays: Vec<Replay>,
    pub stats: Vec<Stat>,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, i32>>,
    pub toons: Vec<Toon>,
}
