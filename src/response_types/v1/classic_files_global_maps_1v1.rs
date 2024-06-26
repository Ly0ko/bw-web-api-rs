use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapAttributes {
    map_candidate: String,
    map_description: String,
    map_era: String,
    map_height: String,
    map_md5: String,
    map_name: String,
    map_path: String,
    map_version: String,
    map_width: String,
    replay_humans: String,
    replay_max_players: String,
    replay_min_players: String,
    replay_opponents: String,
    season_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapInfo {
    attribute: MapAttributes,
    content_size: u64,
    content_type: String,
    md5: String,
    modified_epoch: u64,
    name: String,
    url: String,
}

pub type ClassicFilesGlobalMaps1v1Response = Vec<MapInfo>;
