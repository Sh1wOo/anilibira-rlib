use reqwest::Client;
use anyhow::Result;
use crate::title::types::Title;

mod get_title;

pub struct TitleService {
    client: Client,
}

impl TitleService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_title(&self, query: &str) -> Result<Title> {
        get_title::get_title(&self.client, query).await
    }
}