use reqwest::Client;
use anyhow::{Context, Result};
use json::JsonValue;
use crate::title::types::{Title, Names, Blocked};
use crate::constants::GET_TITLE_URL;

pub async fn get_title(query: &str) -> Result<Title> {
    let client = Client::new();
    let url = format!("{}?{}={}", GET_TITLE_URL, if query.chars().all(char::is_numeric) { "id" } else { "code" }, query);

    let response = client.get(&url)
        .send()
        .await
        .context("Failed to send request")?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response.text().await.context("Failed to get error response body")?;
        return Err(anyhow::anyhow!("API returned error status: {}. Body: {}", status, error_body));
    }

    let body = response.text().await.context("Failed to get response body")?;
    let json = json::parse(&body).context("Failed to parse JSON response")?;

    Ok(parse_title(&json)?)
}

fn parse_title(json: &JsonValue) -> Result<Title> {
    Ok(Title {
        id: json["id"].as_i32().unwrap_or_default(),
        code: json["code"].as_str().unwrap_or_default().to_string(),
        names: Names {
            ru: json["names"]["ru"].as_str().unwrap_or_default().to_string(),
            en: json["names"]["en"].as_str().unwrap_or_default().to_string(),
        },
        description: json["description"].as_str().map(String::from),
        blocked: if json["blocked"].is_object() {
            Some(Blocked {
                blocked: json["blocked"]["blocked"].as_bool().unwrap_or_default(),
                bakanim: json["blocked"]["bakanim"].as_bool().unwrap_or_default(),
            })
        } else {
            None
        },
    })
}