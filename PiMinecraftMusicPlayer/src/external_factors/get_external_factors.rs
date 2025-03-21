use super::{
    get_market_data, get_mercury_retrograde, get_time,
    get_weather_data::{self, get_weather_data},
};

pub struct ExternalFactors {
    pub weather: get_weather_data::Weather,
    pub time: get_time::Time,
    pub market: get_market_data::Market,
    pub mercury_retrograde: bool,
}

pub async fn get_external_factors() -> ExternalFactors {
    ExternalFactors {
        weather: get_weather_data().await.unwrap(),
        time: get_time::get_pacific_time(),
        market: get_market_data::get_market_data().await.unwrap(),
        mercury_retrograde: get_mercury_retrograde::get_mercury_retrograde()
            .await
            .is_ok(),
    }
}
