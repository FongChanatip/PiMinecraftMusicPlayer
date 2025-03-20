
use serde_json::Value;

pub async fn get_mercury_retrograde() -> Result<(bool), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://mercuryretrogradeapi.com")
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&body)?;
    let v = &v["is_retrograde"].as_bool().unwrap();

    Ok(*v)
}
