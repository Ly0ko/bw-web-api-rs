# BW Web API (rust edition)

A Rust library for interacting with the StarCraft Brood War API.

This is a port of the [bw-web-api](https://github.com/evanandrewrose/bw-web-api) library, which is written in TypeScript. You can find more information there such as the expected shape of responses, etc.

When logged in to Starcraft: Remastered, StarCraft.exe creates a local web server that exposes these endpoints. These endpoints are used when
exploring the ladder, viewing profiles, etc.

Below is a table of the known, supported endpoints and the corresponding methods exposed on the `SCApi` struct.

| Endpoint                                                                      | `SCApi` method                                              | Notes                                        |
| ----------------------------------------------------------------------------- | ----------------------------------------------------------- | -------------------------------------------- |
| `/v1/file-set/classic.files.global.maps-1v1`                                  | `classic_files_global_maps_1v1()`                               | List of ladder maps by season                |
| `/v1/gateway`                                                                 | `gateway()`                                                 | Gateways, ids, online player counts          |
| `/v1/leaderboard/{ladder}?offset={offset}&length={length}`                    | `leaderboard_entity(ladder, offset, length)`                 | Paginated player rankings                    |
| `/v1/leaderboard-name-search/{ladder}/{toon}`                                 | `leaderboard_name_search(toon)`                               | Name search of ranked players                |
| `/v1/leaderboard-rank-by-toon/{ladder}/{toon}/{gateway}`                      | `leaderboard_rank_by_toon(ladder, toon, gateway)`              | Ranked data for a given profile              |
| `/v1/leaderboard`                                                             | `leaderboard()`                                             | List of all leaderboards                     |
| `/v1/map-stats-by-toon/{toon}/{gateway}`                                      | `map_stats_by_toon(toon, gateway)`                             | Win rate by map and season                   |
| `/v1/matchmaker-gameinfo-playerinfo/{matchId}`                                | `match_maker_game_info_player_info(matchId)`                     | Ranked match info and link to replay         |
| `/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_mmgameloading` | `aurora_profile_by_toon(toon, gateway, 'scr_mmgameloading')` | Minimal acct info                            |
| `/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_mmtooninfo`    | `aurora_profile_by_toon(toon, gateway, 'scr_mmtooninfo')`    | Minimal acct info + recent game played count |
| `/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_profile`       | `aurora_profile_by_toon(toon, gateway, 'scr_profile')`       | Full acct info                               |
| `/v2/aurora-profile-by-toon/{toon}/{gateway}?request_flags=scr_tooninfo`      | `aurora_profile_by_toon(toon, gateway, 'scr_tooninfo')`      | Full acct info minus game history            |

# Installation

`cargo add bw-web-api-rs`

# Usage

```rust
use bw_api::{api::AuroraProfileByToonV2FieldMask, BWClient, Region, SCApi};

#[tokio::main]
async fn main() {
    let client = BWClient::new("http://localhost/:12345").unwrap();
    let api = SCApi::new(client).unwrap();

    let response = api
        .aurora_profile_by_toon(
            "By.SnOw1",
            Region::Korea,
            AuroraProfileByToonV2FieldMask::ScrProfile,
        )
        .await;

    match response {
        Ok(profile) => println!("{:?}", profile),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

# StarCraft Port

The port StarCraft opens for the web API will not always be the same. You can determine the port on Windows via:

(as administrator)

```
(Get-NetTCPConnection -OwningProcess (Get-Process -Name StarCraft | Select-Object -ExpandProperty Id) | Where-Object {$_.State -eq "Listen"} | Sort-Object -Property LocalPort | Select-Object -First 1).LocalPort
```

An example of running this command in Rust:

```rust
use std::process::Command;
use std::str;

fn get_starcraft_port() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("(Get-NetTCPConnection -OwningProcess (Get-Process -Name StarCraft | Select-Object -ExpandProperty Id) | Where-Object { $_.State -eq 'Listen' } | Sort-Object -Property LocalPort | Select-Object -First 1).LocalPort")
        .output()?;

    if output.status.success() {
        let port = str::from_utf8(&output.stdout)?.trim().to_string();
        Ok(port)
    } else {
        let error_message = str::from_utf8(&output.stderr)?;
        Err(Box::from(error_message))
    }
}
```