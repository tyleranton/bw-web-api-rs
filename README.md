# BW Web API (Rust)
[![crates.io](https://img.shields.io/crates/v/bw-web-api-rs.svg)](https://crates.io/crates/bw-web-api-rs)
[![License](https://img.shields.io/crates/l/bw-web-api-rs.svg)](https://github.com/tyleranton/bw-web-api-rs/blob/master/LICENSE)

A Rust client library for interacting with the StarCraft: Remastered API.

## Features
- Strongly typed API client
- Complete coverage of available endpoints
- Async/await support
- Comprehensive error handling

## Installation

```toml
[dependencies]
bw-web-api-rs = "0.2"
```

Or via cargo:

```bash
cargo add bw-web-api-rs
```

## Usage

This library uses async/await and requires an async runtime such as [tokio](https://github.com/tokio-rs/tokio).

```rust
use bw_web_api_rs::{
    ApiClient, ApiConfig,
    types::{Gateway, Leaderboard},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ApiConfig {
        base_url: "http://127.0.0.1:37843".to_string(),
        api_key: None
    };
    let client = ApiClient::new(config)?;
    
    // Get player match info
    let match_info = client
        .get_matchmaker_player_info("MM-EBECDFA6-B0F4-11EF-8534-FA167A7650A3".to_string())
        .await?;

    // Get player profile (scr_profile field mask)
    let profile = client
        .get_aurora_profile_by_toon_scr_profile("By.SnOw1".to_string(), Gateway::Korea)
        .await?;

    // Fetch leaderboard metadata such as available seasons
    let leaderboard_metadata = client.get_leaderboard().await?;

    Ok(())
}
```

## Available Endpoints

- `get_aurora_profile_by_toon_scr_profile(toon, gateway)` (and other field-mask variants) - Get player profile information
- `get_global_maps_1v1()` - Get available 1v1 maps
- `get_gateway_status()` - Get status of all gateways
- `get_leaderboard()` - Get leaderboard metadata including available gamemodes, gateways, and leaderboards
- `get_leaderboard_entity(leaderboard_id)` - Get leaderboard entries for a specific leaderboard
- `get_leaderboard_rank(leaderboard_id, toon, gateway)` - Get specific player's ranking
- `search_leaderboard_by_battletag(leaderboard_id, battletag)` - Search a leaderboard for players matching a Battletag
- `get_map_stats(toon, gateway)` - Get player's map statistics
- `get_matchmaker_gameinfo(toon, gateway, gamemode, season, offset, limit)` - Get player's match history
- `get_matchmaker_player_info(match_id)` - Get detailed match information

### Field Masks for Aurora Profile
- `ScrProfile` - Basic profile information
- `ScrMmGameLoading` - Matchmaking game loading information
- `ScrMmToonInfo` - Detailed matchmaking information
- `ScrToonInfo` - Basic character information

## StarCraft Port Detection

To find the port StarCraft: Remastered is using on Windows (requires administrator privileges):

```powershell
(Get-NetTCPConnection -OwningProcess (Get-Process -Name StarCraft | Select-Object -ExpandProperty Id) | Where-Object {$_.State -eq "Listen"} | Sort-Object -Property LocalPort | Select-Object -First 1).LocalPort
```

## Error Handling

The library provides a comprehensive error type `ApiError` that covers:
- Network errors
- API errors
- Deserialization errors
- Invalid parameters

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
