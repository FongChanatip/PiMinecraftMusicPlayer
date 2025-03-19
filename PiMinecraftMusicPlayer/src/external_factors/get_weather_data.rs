use dotenv::dotenv;
use reqwest::Error;

pub async fn get_weather_data() -> Result<(), Error> {
    dotenv().ok();

    let lat: f64 = std::env::var("LAT")
        .expect("LAT must be set.")
        .parse()
        .unwrap();
        
    let long: f64 = std::env::var("LONG")
        .expect("LONG must be set.")
        .parse()
        .unwrap();

    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,is_day,cloud_cover,precipitation,rain,weather_code,wind_direction_10m&forecast_days=1", lat, long);
    let res = reqwest::get(url).await?.text().await?;

    println!("{res}");

    Ok(())
}
