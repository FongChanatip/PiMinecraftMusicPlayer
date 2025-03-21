use reqwest::{Client, header::USER_AGENT};
use serde_json::Value;

#[derive(Debug)]
pub struct Market {
    pub spy: f32,
    pub btc: f32,
}

pub async fn get_market_data() -> Result<Market, Box<dyn std::error::Error>> {
    let spy_url = "https://query1.finance.yahoo.com/v8/finance/chart/SPY?region=US&lang=en-US&includePrePost=false&interval=2m&useYfid=true&range=1d%60";
    let btc_url = "https://query1.finance.yahoo.com/v8/finance/chart/BTC-USD?region=US&lang=en-US&includePrePost=false&interval=2m&useYfid=true&range=1d%60";

    let client = Client::new();

    let res = client
        .get(spy_url)
        .header(USER_AGENT, "raspberry-pi")
        .send()
        .await?;

    let body = res.text().await.unwrap();
    let v_spy: Value = serde_json::from_str(&body)?;

    let res = client
        .get(btc_url)
        .header(USER_AGENT, "raspberry-pi")
        .send()
        .await?;

    let body = res.text().await.unwrap();
    let v_btc: Value = serde_json::from_str(&body)?;

    let v_spy_cur_price = v_spy["chart"]["result"][0]["meta"]["regularMarketPrice"]
        .as_f64()
        .ok_or("Missing spy cur price")? as f32;
    let v_spy_prev_close_price = v_spy["chart"]["result"][0]["meta"]["previousClose"]
        .as_f64()
        .ok_or("Missing spy prev close")? as f32;

    let v_btc_cur_price = v_btc["chart"]["result"][0]["meta"]["regularMarketPrice"]
        .as_f64()
        .ok_or("Missing btc cur price")? as f32;
    let v_btc_prev_close_price = v_btc["chart"]["result"][0]["meta"]["previousClose"]
        .as_f64()
        .ok_or("Missing btc prev close")? as f32;

    let spy_percentage =
        ((v_spy_cur_price - v_spy_prev_close_price) / (v_spy_prev_close_price)) * 100.;
    let btc_percentage =
        ((v_btc_cur_price - v_btc_prev_close_price) / (v_btc_prev_close_price)) * 100.;

    let market = Market {
        spy: spy_percentage,
        btc: btc_percentage,
    };

    Ok(market)
}
