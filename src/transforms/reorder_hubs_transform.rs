use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;
use crate::models::{MediaContainer, MetaData};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

use super::Transform;

#[derive(Default, Debug)]
pub struct ReorderHubsTransform;

#[async_trait]
impl Transform for ReorderHubsTransform {
    async fn transform_mediacontainer(
        &self,
        item: &mut MediaContainer,
        _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        let config = Config::load();

        let mut reordered_hubs: Vec<MetaData> = Vec::new();

        // Move in_progress and next_up hubs to the top if they exist
        if config.better_on_deck {
            if let Some(in_progress) = &config.in_progress {
                move_to_top(
                    in_progress,
                    item.children_mut(),
                    &mut reordered_hubs,
                );
            }
            if let Some(next_up) = &config.next_up {
                move_to_top(next_up, item.children_mut(), &mut reordered_hubs);
            }
        }

        // Move other priority hubs to the top
        if let Some(priority_titles) = &config.priority_hubs {
            for title in priority_titles {
                move_to_top(title, item.children_mut(), &mut reordered_hubs);
            }
        }

        // Add remaining hubs
        for hub in item.children_mut() {
            if !reordered_hubs.contains(hub) {
                reordered_hubs.push(hub.clone());
            }
        }

        item.set_children(reordered_hubs);
        Ok(())
    }
}

fn move_to_top(
    title: &str,
    hubs: &[MetaData],
    reordered_hubs: &mut Vec<MetaData>,
) {
    for hub in hubs.iter().filter(|h| h.title == title).cloned() {
        if !reordered_hubs.contains(&hub) {
            reordered_hubs.push(hub);
        }
    }
}
