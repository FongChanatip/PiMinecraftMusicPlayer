use super::{get_market_data, get_mercury_retrograde, get_time, get_weather_data::{self, get_weather_data}};

pub struct ExternalFactors {
    pub weather: get_weather_data::Weather,
    pub time: get_time::Time,
    pub market: get_market_data::Market,
    pub mercury_retrograde: bool,
}

pub async fn get_external_factors() -> ExternalFactors {
    ExternalFactors { 
        weather: get_weather_data().await.unwrap(), 
        time: get_time::Time { min: 0, hour: 0, day: 1, month: 0, year: 2025, season: "winter".to_string() }, 
        market: get_market_data::Market { spy: 1.0, btc: 1.0 }, 
        mercury_retrograde: false 
    }
}