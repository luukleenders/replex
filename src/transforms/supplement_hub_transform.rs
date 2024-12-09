use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;
use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::CollectionChildren;
use crate::transforms::Transform;

#[derive(Default, Debug)]
pub struct SupplementHubTransform;

#[async_trait]
impl Transform for SupplementHubTransform {
    async fn transform_mediacontainer(
        &self,
        container: &mut MediaContainer,
        plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if container.hub.is_empty() {
            return Ok(());
        }

        let config = Config::load();

        for hub in &mut container.hub {
            if hub.size.unwrap_or_default() == 0 {
                continue;
            }

            let mut is_supplemented = false;

            // In order to manually sort the better_on_deck hubs, we need to get all the children in one go
            if let (Some(in_progress), Some(next_up)) = (
                &config.better_on_deck.in_progress,
                &config.better_on_deck.next_up,
            ) {
                if hub.title == *in_progress || hub.title == *next_up {
                    if let Some(id) = id_from_key(hub.key.as_ref().unwrap()) {
                        let mut children =
                            CollectionChildren::get(plex_client, id, None, None)
                                .await
                                .unwrap();

                        hub.metadata = children.children();
                        is_supplemented = true;
                    }
                }
            }

            // If the previous step removed items from the hub, we need to supplement it
            if !is_supplemented {
                if let Some(size) = hub.size {
                    if size > hub.children().len() as i32 {
                        if let Some(id) = id_from_key(hub.key.as_ref().unwrap()) {
                            let offset = hub.children().len() as i32;
                            let limit = size - offset;
                            let mut children = CollectionChildren::get(
                                plex_client,
                                id,
                                Some(size),
                                Some(limit),
                            )
                            .await?;

                            hub.metadata.append(&mut children.children())
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn id_from_key(key: &str) -> Option<i64> {
    let key_string = key
        .replace("/hubs/library/collections/", "")
        .replace("/library/collections/", "")
        .replace("/children", "")
        .replace('/', "");

    key_string.parse::<i64>().ok()
}
