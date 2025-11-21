use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_airnow(client: &Client, api_key: &str, lat: f64, lon: f64) -> Result<Value> {
    let url = format!(
        "https://www.airnowapi.org/aq/observation/latLong/current/?format=application/json&latitude={}&longitude={}&distance=25&API_KEY={}",
        lat, lon, api_key
    );

    let resp = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let v = resp.json::<Value>().await?;
    Ok(v)
}

/// Fetch current observations for a US ZIP code from AirNow.
pub async fn fetch_airnow_by_zip(client: &Client, api_key: &str, zip: &str) -> Result<Value> {
    let url = format!(
        "https://www.airnowapi.org/aq/observation/zipCode/current/?format=application/json&zipCode={}&distance=25&API_KEY={}",
        zip, api_key
    );

    let resp = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let v = resp.json::<Value>().await?;
    Ok(v)
}
