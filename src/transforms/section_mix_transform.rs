use anyhow::Result;
use async_trait::async_trait;
use itertools::Itertools;

use crate::models::{MediaContainer, MetaData};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::{Collection, CollectionChildren};
use crate::transforms::Transform;

/// Merge children of all collections in `collection_ids` into a single collection
#[derive(Default, Debug)]
pub struct SectionMixTransform {
    pub collection_ids: Vec<i64>,
    pub offset: i32,
    pub limit: i32,
}

#[async_trait]
impl Transform for SectionMixTransform {
    async fn transform_mediacontainer(
        &self,
        container: &mut MediaContainer,
        plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        let mut total_children: Vec<MetaData> = vec![];
        let mut total_size: i32 = 0;

        if self.collection_ids.is_empty() {
            return Ok(());
        }

        // These are (or should be anyway) the same for all collections so we can just use the first one
        let first_id = self.collection_ids[0];
        let mut collection = match Collection::get(plex_client, first_id).await
        {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(error = %e, "Failed to get collection");
                return Err(e);
            }
        };
        let exclude_watched = collection.exclude_watched();
        let children = collection.children();
        let collection_title = children.first().unwrap().title.clone();

        dbg!(&collection_title);

        // Get all children for each collection
        for &id in &self.collection_ids {
            let (limit, offset) = if !exclude_watched {
                (self.limit, self.offset)
            } else {
                (250, 0)
            };

            let mut children = CollectionChildren::get(
                plex_client,
                id,
                Some(offset),
                Some(limit),
            )
            .await
            .unwrap();

            let lenght = children.children().len() as i32;

            if exclude_watched {
                children.children_mut().retain(|c| !c.is_watched());
            }

            let difference = lenght - children.children().len() as i32;

            dbg!(difference);

            total_size += children.children().len() as i32;

            if !total_children.is_empty() {
                total_children = total_children
                    .into_iter()
                    .interleave(children.children())
                    .collect()
            } else {
                total_children.append(&mut children.children());
            }
        }

        container.offset = Some(self.offset);
        container.size = Some(self.limit as i64);
        container.total_size = Some(total_size);
        container.metadata = total_children;
        container
            .better_on_deck(&collection_title, plex_client)
            .await;

        Ok(())
    }
}
