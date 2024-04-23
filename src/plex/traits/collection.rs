use anyhow::Result;
use async_trait::async_trait;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;

#[async_trait]
pub trait Collection {
    async fn get(&self, id: i64) -> Result<MediaContainer>;
}

#[async_trait]
impl Collection for PlexClient {
    async fn get(&self, id: i64) -> Result<MediaContainer> {
        let cache_name = format!("collection:{}", id);
        let cache_key = self.generate_cache_key(cache_name);
        let path = format!("/library/collections/{}", id);

        Self::cache_or_fetch(&cache_key, || async {
            let res = self.get(&path).await.map_err(|e| {
                anyhow::anyhow!("Failed to get library collection: {}", e)
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
