# BW Web API (rust edition)

A Rust library for interacting with the StarCraft Brood War API.

This is a port of the [bw-web-api](https://github.com/evanandrewrose/bw-web-api) library, which is written in TypeScript. You can find more information there such as endpoint documentation, the expected shape of responses, etc.

Not every endpoint is working yet and this is still a work in progress. However, the most useful endpoints are implemented and working (eg. `aurora_profile_by_toon`).

# Installation

`cargo add bw-web-api-rs`

# Usage

```rust
use bw_api::{AuroraProfileByToonV2FieldMask, BWClient, Region, SCApi};

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