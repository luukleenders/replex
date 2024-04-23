use anyhow::Result;
use async_trait::async_trait;

use crate::models::{MetaData, Style};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

use super::Transform;

#[derive(Default, Debug)]
pub struct HubKeyTransform;

#[async_trait]
impl Transform for HubKeyTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if item.is_hub()
            && item.key.is_some()
            && !item.key.as_ref().unwrap().starts_with("/replex")
        {
            let style = item
                .style
                .clone()
                .unwrap_or(Style::Shelf.to_string().to_lowercase());
            let key = item.key.clone().unwrap_or_default();

            item.key = Some(format!("/replex/{}{}", style, key));

            tracing::debug!("Transformed hub key: {:?}", item.key);
        }

        return Ok(());
    }
}
