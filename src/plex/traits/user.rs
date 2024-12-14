use std::str;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::plex::client::PlexClient;

#[async_trait]
pub trait User {
    async fn get(&self, token: String) -> Result<String>;
}

#[async_trait]
impl User for PlexClient {
    async fn get(&self, token: String) -> Result<String> {
        let cache_name = format!("username:{}", token);
        let cache_key = self.generate_cache_key(cache_name);
        let path = String::from("https://clients.plex.tv/api/v2/user");

        let mut headers = http::HeaderMap::new();
        headers.insert(
            "X-Plex-Token",
            http::HeaderValue::from_str(&token)?,
        );

        Self::cache_or_fetch(&cache_key, || async {
            let res = self.get(&path, Some(headers)).await.map_err(|e| {
                anyhow::anyhow!("Error fetching user: {}", e)
            })?;

            let bytes = res.bytes().await.unwrap();
            let response = str::from_utf8(&bytes).unwrap_or_else(|_| "error");

            let json: Value = serde_json::from_str(response).map_err(|e| {
                anyhow::anyhow!("Error parsing JSON: {}", e)
            })?;

            let username = json["username"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();

            return Ok(username);
        })
            .await
    }
}