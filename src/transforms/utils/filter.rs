use crate::models::{MediaContainer, MetaData};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Filter: Send + Sync {
    async fn filter_metadata(
        &self, _item: &mut MetaData, _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> bool {
        // Default implementation that does nothing and just returns true
        true
    }

    async fn filter_mediacontainer(
        &self, _item: &MediaContainer, _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<bool> {
        // Default implementation that does nothing and just returns Ok(true)
        Ok(true)
    }
}
