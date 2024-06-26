use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::BWClient;
use crate::response_types::{v1::*, v2::*};
use crate::error::ApiError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuroraProfileByToonV2FieldMask {
    ScrMmGameLoading,
    ScrMmToonInfo,
    ScrProfile,
    ScrToonInfo,
}

impl std::fmt::Display for AuroraProfileByToonV2FieldMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuroraProfileByToonV2FieldMask::ScrMmGameLoading => write!(f, "scr_mmgameloading"),
            AuroraProfileByToonV2FieldMask::ScrMmToonInfo => write!(f, "scr_mmtooninfo"),
            AuroraProfileByToonV2FieldMask::ScrProfile => write!(f, "scr_profile"),
            AuroraProfileByToonV2FieldMask::ScrToonInfo => write!(f, "scr_tooninfo"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuroraProfileByToonResponse {
    ScrMmGameLoading(AuroraProfileByToonScrMmGameLoadingResponse),
    ScrMmToonInfo(AuroraProfileByToonScrMmToonInfoResponse),
    ScrProfile(AuroraProfileByToonScrProfileResponse),
    ScrToonInfo(AuroraProfileByToonScrToonInfoResponse),
}

#[repr(i32)]
#[derive(Debug)]
pub enum Region {
    USWest = 10,
    USEast = 11,
    Europe = 20,
    Korea = 30,
    Asia = 45,
}

#[repr(i32)]
#[derive(Debug)]
pub enum Leaderboard {
    Global = 12960,
    USWest = 12961,
    USEast = 12962,
    Europe = 12963,
    Korea = 12964,
    Asia = 12965,
}

pub struct SCApi {
    client: BWClient,
}

impl SCApi {
    pub fn new(client: BWClient) -> Result<Self, ApiError> {
        Ok(Self { client })
    }

    async fn schema_fetch<T: for<'de> Deserialize<'de> + Serialize>(&self, path: &str) -> Result<Value, ApiError> {
        let response_text = self.client.fetch(path).await?;
        let response: T = serde_json::from_str(&response_text)?;
        let response_json = serde_json::to_value(&response)?;
        Ok(response_json)
    }

    pub async fn classic_files_global_maps_1v1(&self) -> Result<Value, ApiError> {
        self.schema_fetch::<ClassicFilesGlobalMaps1v1Response>("web-api/v1/file-set/classic.files.global.maps-1v1").await
    }

    pub async fn gateway(&self) -> Result<Value, ApiError> {
        self.schema_fetch::<GatewayResponse>("web-api/v1/gateway").await
    }

    pub async fn leaderboard(&self) -> Result<Value, ApiError> {
        self.schema_fetch::<LeaderboardResponse>("web-api/v1/leaderboard").await
    }

    pub async fn leaderboard_entity(&self, leaderboard: Leaderboard, offset: i32, length: i32) -> Result<Value, ApiError> {
        if offset < 0 {
            return Err(ApiError::InvalidInput("offset must be >= 0".to_string()));
        }
        if length > 100 {
            return Err(ApiError::InvalidInput("length must be <= 100".to_string()));
        }
        let path = format!("web-api/v1/leaderboard/{}?offset={}&length={}", leaderboard as i32, offset, length);
        self.schema_fetch::<LeaderboardEntityResponse>(&path).await
    }

    pub async fn leaderboard_name_search(&self, leaderboard: Leaderboard, toon: &str) -> Result<Value, ApiError> {
        let path = format!("web-api/v1/leaderboard-name-search/{}/{}", leaderboard as i32, urlencoding::encode(toon));
        self.schema_fetch::<LeaderboardNameSearchResponse>(&path).await
    }

    pub async fn leaderboard_rank_by_toon(&self, leaderboard: Leaderboard, toon: &str, gateway: Region) -> Result<Value, ApiError> {
        let path = format!("web-api/v1/leaderboard-rank-by-toon/{}/{}/{}", leaderboard as i32, urlencoding::encode(toon), gateway as i32);
        self.schema_fetch::<LeaderboardRankByToonResponse>(&path).await
    }

    pub async fn map_stats_by_toon(&self, toon: &str, gateway: Region) -> Result<Value, ApiError> {
        let path = format!("web-api/v1/map-stats-by-toon/{}/{}", urlencoding::encode(toon), gateway as i32);
        self.schema_fetch::<MapStatsByToonResponse>(&path).await
    }

    pub async fn match_maker_game_info_by_toon(&self, toon: &str, gateway: Region, game_mode: i32, season: i32, offset: i32, limit: i32) -> Result<Value, ApiError> {
        let path = format!("web-api/v1/matchmaker-gameinfo-by-toon/{}/{}/{}/{}?offset={}&limit={}", urlencoding::encode(toon), gateway as i32, game_mode, season, offset, limit);
        self.schema_fetch::<MatchMakerGameInfoByToonResponse>(&path).await
    }

    pub async fn match_maker_game_info_player_info(&self, match_id: &str) -> Result<Value, ApiError> {
        let path = format!("web-api/v1/matchmaker-gameinfo-playerinfo/{}", urlencoding::encode(match_id));
        self.schema_fetch::<MatchMakerGameInfoByToonResponse>(&path).await
    }

    pub async fn aurora_profile_by_toon(&self, toon: &str, gateway: Region, mask: AuroraProfileByToonV2FieldMask) -> Result<Value, ApiError> {
        let mask_str = mask.to_string();

        let path = format!("web-api/v2/aurora-profile-by-toon/{}/{}?request_flags={}", urlencoding::encode(toon), gateway as i32, mask_str);

        match mask {
            AuroraProfileByToonV2FieldMask::ScrMmGameLoading => {
                self.schema_fetch::<AuroraProfileByToonScrMmGameLoadingResponse>(&path).await
            },
            AuroraProfileByToonV2FieldMask::ScrMmToonInfo => {
                self.schema_fetch::<AuroraProfileByToonScrMmToonInfoResponse>(&path).await
            },
            AuroraProfileByToonV2FieldMask::ScrProfile => {
                self.schema_fetch::<AuroraProfileByToonScrProfileResponse>(&path).await
            },
            AuroraProfileByToonV2FieldMask::ScrToonInfo => {
                self.schema_fetch::<AuroraProfileByToonScrToonInfoResponse>(&path).await
            },
        }
    }
}
