mod airnow_apis;
mod storage;

use anyhow::Result;
use log::info;
use std::path::Path;
use std::io::{self, Write};

// Prompt the user and return a trimmed string.
fn prompt_string(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Read an environment variable or prompt the user with a default value.
fn env_or_prompt(env_key: &str, prompt_label: &str, default: &str) -> Result<String> {
    if let Ok(val) = std::env::var(env_key) {
        if !val.is_empty() {
            return Ok(val);
        }
    }

    let full_prompt = format!("{} [{}]: ", prompt_label, default);
    let input = prompt_string(&full_prompt)?;
    if input.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(input)
    }
}

// Simple ZIP validator: US 5-digit ZIP codes only
fn is_valid_zip(zip: &str) -> bool {
    let s = zip.trim();
    s.len() == 5 && s.chars().all(|c| c.is_ascii_digit())
}

// Prompt until a valid ZIP is provided (uses env_or_prompt for default/env)
fn prompt_valid_zip() -> Result<String> {
    loop {
        let zip = env_or_prompt("ZIP", "ZIP", &std::env::var("ZIP").unwrap_or_else(|_| "20001".into()))?;
        if is_valid_zip(&zip) {
            return Ok(zip);
        }
        println!("Invalid ZIP code '{}'. Please enter a 5-digit US ZIP code.", zip);
    }
}

// Prompt until valid lat/lon are provided and return parsed values
fn prompt_valid_latlon() -> Result<(f64, f64)> {
    loop {
        let lat_str = env_or_prompt("LAT", "Latitude", &std::env::var("LAT").unwrap_or_else(|_| "38.9072".into()))?;
        let lon_str = env_or_prompt("LON", "Longitude", &std::env::var("LON").unwrap_or_else(|_| "-77.0369".into()))?;

        match (lat_str.parse::<f64>(), lon_str.parse::<f64>()) {
            (Ok(lat), Ok(lon)) if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) => {
                return Ok((lat, lon));
            }
            _ => println!("Invalid coordinates. Latitude must be between -90 and 90; Longitude between -180 and 180."),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let api_key = std::env::var("AIRNOW_API_KEY")
        .expect("Please set AIRNOW_API_KEY environment variable");

    // default output file (can be overriden with OUT env var)
    let out = std::env::var("OUT").unwrap_or_else(|_| "airnow.json".into());

    let choice = env_or_prompt("QUERY_MODE", "Query by (1) lat/lon or (2) ZIP? [1/2]", "1")?;

    let client = reqwest::Client::new();

    if choice == "2" || choice.eq_ignore_ascii_case("zip") {
        // ZIP flow with validation
        let zip = prompt_valid_zip()?;
        info!("Fetching AirNow data for zip={}", zip);
        let data = airnow_apis::fetch_airnow_by_zip(&client, &api_key, &zip).await?;
        storage::save_to_file(Path::new(&out), &data)?;
    } else {
        // lat/lon flow with validation
        let (lat, lon) = prompt_valid_latlon()?;
        info!("Fetching AirNow data for lat={}, lon={}", lat, lon);
        let data = airnow_apis::fetch_airnow(&client, &api_key, lat, lon).await?;
        storage::save_to_file(Path::new(&out), &data)?;
    }

    info!("Saved AirNow data to {}", out);

    Ok(())
}
