use anyhow::Result;
use async_trait::async_trait;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

use super::Transform;

#[derive(Default, Debug)]
pub struct ExcludeWatchedTransform;

#[async_trait]
impl Transform for ExcludeWatchedTransform {
    async fn transform_mediacontainer(
        &self, container: &mut MediaContainer, plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if container.hub.is_empty() {
            return Ok(());
        }

        for hub in &mut container.hub {
            if hub.exclude_watched(plex_client).await.unwrap_or(false) {
                hub.children_mut().retain(|x| !x.is_watched());
            }
        }

        Ok(())
    }
}
