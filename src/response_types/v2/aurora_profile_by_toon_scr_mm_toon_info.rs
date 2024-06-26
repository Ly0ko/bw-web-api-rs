use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuroraProfileByToonScrMmToonInfoResponse {
    PlayerFound(AuroraProfileByToonScrMmToonInfoResponsePlayerFound),
    PlayerNotFound(AuroraProfileByToonScrMmToonInfoResponsePlayerNotFound),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuroraProfileByToonScrMmToonInfoResponsePlayerFound {
    pub account_flags: Option<String>,
    pub aurora_id: i32,
    pub battle_tag: String,
    pub country_code: String,
    pub matchmaked_current_season: i32,
    pub matchmaked_current_season_buckets: Vec<i32>,
    pub matchmaked_stats: Vec<MatchmakedStats>,
    pub program_id: String,
    pub toon_guid_by_gateway: HashMap<String, HashMap<String, i32>>,
    pub toons: Vec<Toon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuroraProfileByToonScrMmToonInfoResponsePlayerNotFound {
    pub aurora_id: i32,
    pub avatars: HashMap<String, String>,
    pub avatars_framed: HashMap<String, AvatarFramed>,
    pub avatars_unlocked: HashMap<String, AvatarUnlocked>,
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
