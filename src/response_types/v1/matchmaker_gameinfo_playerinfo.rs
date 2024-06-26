use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchMakerGameInfoByToonResponse(Vec<HashMap<String, MatchInfo>>);

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchInfo {
    pub match_created: String,
    pub players: Vec<PlayerInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    pub aurora_id: i32,
    pub avatar_url: String,
    pub benefactor_id: String,
    pub game_info: Option<GameInfo>,
    pub game_result: Option<HashMap<String, GameResult>>,
    pub gateway_id: i32,
    pub info_attributes: InfoAttributes,
    pub is_winner: String,
    pub matching_attributes: MatchingAttributes,
    pub name: String,
    pub score: Score,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    pub attributes: GameAttributes,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameAttributes {
    pub closed_slots: String,
    pub flags: String,
    pub game_speed: String,
    pub host_name: String,
    pub is_replay: String,
    pub map_crc: String,
    pub map_file_name: String,
    pub map_file_size: String,
    pub map_height: String,
    pub map_md5: String,
    pub map_name: String,
    pub map_tile_set: String,
    pub map_width: String,
    pub net_turn_rate: String,
    pub observers_current: String,
    pub observers_max: String,
    pub players_ai: String,
    pub players_current: String,
    pub players_max: String,
    pub proxy: Option<String>,
    pub rank: Option<String>,
    pub save_game_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResult {
    pub attributes: GameResultAttributes,
    pub is_computer: Option<bool>,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResultAttributes {
    pub g_player_data_idx: String,
    pub left: String,
    pub race: Option<String>,
    pub team: Option<String>,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoAttributes {
    pub map: String,
    pub map_selection: Option<String>,
    pub player_battle_tag: Option<String>,
    pub player_legacy_gateway_id: Option<String>,
    pub player_legacy_toon_name: Option<String>,
    pub player_region: Option<String>,
    pub player_routing_via_proxy_server: Option<String>,
    pub race: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchingAttributes {
    pub net_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub base: i32,
    pub bucket_new: i32,
    pub bucket_old: i32,
    pub delta: i32,
    pub win_streak: i32,
}
