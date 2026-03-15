use reqwest::Client;
use serde_json::json;

use crate::config::settings::Settings;


pub async fn llm(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {


    let settings = Settings::load();

    let settings = Settings::load();

    let url = format!(
        "{}/models/{}:generateContent?key={}",
        settings.llm_base_url,
        settings.llm_model,
        settings.llm_api_key
    );

    let body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let client = Client::new();

    let res = client
        .post(url)
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;

    let output = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("No response")
        .to_string();

    println!("{}", output);

    Ok(output)

}