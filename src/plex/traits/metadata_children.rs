use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;

#[async_trait]
pub trait MetaDataChildren {
    async fn get(&self, id: &str) -> anyhow::Result<MediaContainer>;
}

#[async_trait]
impl MetaDataChildren for PlexClient {
    async fn get(&self, id: &str) -> Result<MediaContainer> {
        let cache_name = format!("metadata_children:{}", id);
        let cache_key = self.generate_cache_key(cache_name);
        let path = build_path(id);

        Self::cache_or_fetch(&cache_key, || async {
            let res = self.get(&path, None).await.map_err(|e| {
                anyhow::anyhow!("Failed to get collection children: {}", e)
            })?;

            MediaContainer::from_reqwest_response(res)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("Error deserializing response: {}", e)
                })
        })
        .await
    }
}

fn build_path(id: &str) -> String {
    let mut params: HashMap<&str, String> = HashMap::new();

    // Always include `includeGuids`
    params.insert("includeGuids", "1".to_string());

    let query_string = serde_urlencoded::to_string(params).unwrap();

    format!("/library/metadata/{}/children?{}", id, query_string)
}
