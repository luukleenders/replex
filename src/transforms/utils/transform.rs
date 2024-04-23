use anyhow::{Ok, Result};
use async_trait::async_trait;

use crate::models::{MediaContainer, MetaData};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

#[async_trait]
pub trait Transform: Send + Sync {
    async fn transform_metadata(
        &self, _item: &mut MetaData, _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        // Default implementation that does nothing and just returns Ok(())
        Ok(())
    }

    // Implementers can choose not to override this method.
    async fn transform_mediacontainer(
        &self, _item: &mut MediaContainer, _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        // Default implementation that does nothing and just returns Ok(())
        Ok(())
    }
}
