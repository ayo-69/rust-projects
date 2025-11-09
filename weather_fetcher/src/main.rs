use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f64,
    humidity: u64,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "267308fc82a9ca4e9176360924068a41";
    let mut city = "Lagos";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;

    println!(
        "City: {}, Temperature: {} C, Humidity: {}%",
        res.name, res.main.temp, res.main.humidity
    );

    Ok(())
}
