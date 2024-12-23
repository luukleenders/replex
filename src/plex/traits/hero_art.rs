use anyhow::Result;
use async_trait::async_trait;

use crate::plex::client::PlexClient;

#[async_trait]
pub trait HeroArt {
    async fn get(&self, uuid: &str) -> anyhow::Result<Option<String>>;
}

#[async_trait]
impl HeroArt for PlexClient {
    // async fn get(&self, guid: &str) -> Result<Option<String>> {
    //     let cache_name = format!("hero_art:{}", guid);
    //     let cache_key = self.generate_cache_key(cache_name);
    //
    //     Self::cache_or_fetch(&cache_key, || async {
    //         if guid.starts_with("local://") {
    //             tracing::debug!("Skipping local item: {}", guid);
    //             return Ok(None);
    //         }
    //
    //         let patterns = ["show/", "movie/", "season/", "episode/"];
    //         let cleaned_guid =
    //             patterns.iter().fold(guid.to_owned(), |acc, pat| {
    //                 acc.replace(&format!("plex://{}", pat), "")
    //             });
    //         let mut container = self.get_provider_data(&cleaned_guid).await?;
    //
    //         let cover_art = container.children().iter().find_map(|meta| {
    //             meta.images
    //                 .iter()
    //                 .find(|image| image.r#type == "coverArt")
    //                 .map(|image| image.url.clone())
    //         });
    //
    //         Ok(cover_art)
    //     })
    //     .await
    // }
    async fn get(&self, uuid: &str) -> Result<Option<String>> {
        println!("HeroArt::get");
        let cache_name = format!("hero_art:{}", uuid);
        let cache_key = self.generate_cache_key(cache_name);

        Self::cache_or_fetch(&cache_key, || async {
            let uuid = uuid.to_string();
            let mut container = self.get_provider_data(&uuid).await?;

            let metadata = container.children_mut().get(0);
            let mut image: Option<String> = None;
            if metadata.is_some() {
                for i in &metadata.unwrap().images {
                    if i.r#type == "coverArt" {
                        image = Some(i.url.clone());
                        break;
                    }
                }
            }
            image.as_ref();
            Ok(image)
        })
        .await
    }
}
