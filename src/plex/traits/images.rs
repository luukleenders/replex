use anyhow::Result;
use async_trait::async_trait;

use crate::models::Image;
use crate::plex::client::PlexClient;

#[async_trait]
pub trait Images {
    async fn get(&self, guid: &str) -> anyhow::Result<Option<Vec<Image>>>;
}

#[async_trait]
impl Images for PlexClient {
    async fn get(&self, guid: &str) -> Result<Option<Vec<Image>>> {
        let cache_name = format!("images:{}", guid);
        let cache_key = self.generate_cache_key(cache_name);

        Self::cache_or_fetch(&cache_key, || async {
            if guid.starts_with("local://") {
                tracing::debug!("Skipping local item: {}", guid);
                return Ok(None);
            }


            let patterns = ["show/", "movie/", "season/", "episode/"];
            let cleaned_guid =
                patterns.iter().fold(guid.to_owned(), |acc, pat| {
                    acc.replace(&format!("{}", pat), "")
                });
            let mut container = self.get_provider_data(&cleaned_guid).await?;

            // let cover_art = container.children().iter().find_map(|meta| {
            //     meta.images
            //         .iter()
            //         .find(|image| image.r#type == "coverArt")
            //         .map(|image| image.url.clone())
            // });

            // let images = Some(container.children().first().unwrap().images.clone());

            if let Some(images) = container.children().first() {
                return Ok(Some(images.images.clone()));
            } else {
                return Ok(None);
            }
        })
        .await
    }
}
