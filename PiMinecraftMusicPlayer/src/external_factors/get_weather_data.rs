use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Weather {
    is_daytime: bool,
    temperature: i16,
    probability_precipitation: f32,
    short_forecast: String,
}

pub async fn get_weather_data() -> Result<Weather, Box<dyn std::error::Error>> {
    dotenv().ok();

    let grid1: i8 = std::env::var("GRID1").expect("add grid 1").parse().unwrap();
    let grid2: i8 = std::env::var("GRID2").expect("add grid 2").parse().unwrap();
    let client = Client::new();

    let url = format!(
        "https://api.weather.gov/gridpoints/SGX/{},{}/forecast/hourly",
        grid1, grid2
    );

    let res = client
        .get(url)
        .header(USER_AGENT, "weather-app")
        .send()
        .await?;
    let body = res.text().await.unwrap();

    let v: Value = serde_json::from_str(&body)?;

    let period = &v["properties"]["periods"][0];
    let is_daytime = period["isDaytime"].as_bool().ok_or("Missing daytime");
    let temperature = period["temperature"].as_i64().ok_or("Missing temp field")? as i16;
    let probability_precipitation = period["probabilityOfPrecipitation"]["value"]
        .as_f64()
        .ok_or("Missing precip")? as f32;
    let short_forecast = period["shortForecast"].to_string();
    let weather = Weather {
        short_forecast,
        is_daytime: is_daytime?,
        temperature,
        probability_precipitation,
    };

    Ok(weather)
}
