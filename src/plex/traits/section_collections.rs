use anyhow::Result;
use async_trait::async_trait;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;

#[async_trait]
pub trait SectionCollections {
    async fn get(&self, id: i64) -> anyhow::Result<MediaContainer>;
}

#[async_trait]
impl SectionCollections for PlexClient {
    async fn get(&self, id: i64) -> Result<MediaContainer> {
        let cache_name = format!("section_collections:{}", id);
        let cache_key = self.generate_cache_key(cache_name);
        let path = format!("/library/sections/{}/collections", id);

        Self::cache_or_fetch(&cache_key, || async {
            let res = self.get(&path).await.map_err(|e| {
                anyhow::anyhow!("Failed to get section collections: {}", e)
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
