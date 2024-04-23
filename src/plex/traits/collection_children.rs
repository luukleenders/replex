use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;

#[async_trait]
pub trait CollectionChildren {
    async fn get(
        &self,
        id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> anyhow::Result<MediaContainer>;
}

#[async_trait]
impl CollectionChildren for PlexClient {
    async fn get(
        &self,
        id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MediaContainer> {
        let cache_name = format!(
            "collection_children:{},offset:{:?},limit:{:?}",
            id, offset, limit
        );
        let cache_key = self.generate_cache_key(cache_name);
        let path = build_path(id, offset, limit);

        Self::cache_or_fetch(&cache_key, || async {
            let res = self.get(&path).await.map_err(|e| {
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

fn build_path(id: i64, offset: Option<i32>, limit: Option<i32>) -> String {
    let mut params: HashMap<&str, String> = HashMap::new();

    // Always include `includeGuids`
    params.insert("includeGuids", "1".to_string());

    // Conditionally include `offset` and `limit`
    if let Some(offset_val) = offset {
        params.insert("X-Plex-Container-Start", offset_val.to_string());
    }

    if let Some(limit_val) = limit {
        params.insert("X-Plex-Container-Size", limit_val.to_string());
    }

    let query_string = serde_urlencoded::to_string(params).unwrap();

    format!("/library/collections/{}/children?{}", id, query_string)
}
