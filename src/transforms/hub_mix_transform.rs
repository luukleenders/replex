use anyhow::Result;
use async_trait::async_trait;
use itertools::Itertools;

use crate::models::{MediaContainer, MetaData};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::transforms::Transform;

#[derive(Default, Debug)]
pub struct HubMixTransform;

#[async_trait]
impl Transform for HubMixTransform {
    async fn transform_mediacontainer(
        &self,
        container: &mut MediaContainer,
        plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if container.hub.is_empty() {
            return Ok(());
        }

        let mut new_hubs: Vec<MetaData> = Vec::new();

        for hub in &container.hub {
            if hub.size.unwrap_or_default() == 0 {
                continue;
            }

            // Find the position of an existing hub with the same title
            let position = new_hubs.iter().position(|v| v.title == hub.title);

            match position {
                Some(pos) => {
                    // Hub with same title exists, merge current hub's content into it
                    let existing_hub = &mut new_hubs[pos];

                    // Merge keys if both exist
                    if let (Some(existing_key), Some(current_key)) =
                        (&existing_hub.key, &hub.key)
                    {
                        // Prepare a slice with references to the keys to merge.
                        let keys_to_merge =
                            [existing_key.as_str(), current_key.as_str()];
                        // Call merge_hub_keys with the slice of keys.
                        existing_hub.key = Some(merge_hub_keys(&keys_to_merge));
                    }

                    // Prepare an iterator over all children, avoiding early cloning.
                    let all_children = existing_hub
                        .children_mut()
                        .iter()
                        .cloned()
                        .interleave(hub.clone().children().iter().cloned())
                        .collect();

                    existing_hub.set_children(all_children);

                    continue;
                }
                None => {
                    // No hub with the same title exists, add the current hub as is
                    new_hubs.push(hub.clone());
                }
            }
        }

        for hub in &mut new_hubs {
            hub.better_on_deck(plex_client).await;
        }

        container.hub = new_hubs;

        Ok(())
    }
}

pub fn merge_hub_keys(keys: &[&str]) -> String {
    // Filter and clean each key, then join them with commas.
    let cleaned_keys: Vec<String> = keys
        .iter()
        .map(|&key| {
            key.replace("/hubs/library/collections/", "")
                .replace("/library/collections/", "")
                .replace("/children", "")
        })
        .collect();

    // Return the formatted key with all IDs merged.
    format!("/library/collections/{}/children", cleaned_keys.join(","))
}
