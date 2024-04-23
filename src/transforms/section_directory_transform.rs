use anyhow::Result;
use async_trait::async_trait;

use crate::models::MetaData;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

use super::Transform;

#[derive(Default, Debug)]
pub struct SectionDirectoryTransform;

#[async_trait]
impl Transform for SectionDirectoryTransform {
    async fn transform_metadata(
        &self, item: &mut MetaData, _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if !item.is_collection_hub() || item.directory.is_empty() {
            return Ok(());
        }

        let childs = item.metadata.clone();
        item.directory = vec![];
        item.video = childs;

        return Ok(());
    }
}
