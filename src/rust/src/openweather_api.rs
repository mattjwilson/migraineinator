use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

/// Fetch current weather from OpenWeatherMap using lat/lon.
pub async fn fetch_openweather_by_latlon(client: &Client, api_key: &str, lat: f64, lon: f64) -> Result<Value> {
    // Using 'metric' units by default. Change or make configurable if needed.
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
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

/// Fetch current weather from OpenWeatherMap using ZIP code (assumes US unless country included in zip string).
pub async fn fetch_openweather_by_zip(client: &Client, api_key: &str, zip: &str) -> Result<Value> {
    // If the zip already contains a country (e.g., "20001,us"), use it directly.
    let zip_param = if zip.contains(',') { zip.to_string() } else { format!("{},us", zip) };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units=metric",
        zip_param, api_key
    );

    let resp = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let v = resp.json::<Value>().await?;
    Ok(v)
}
