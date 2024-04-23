use anyhow::Result;
use futures::executor::block_on;
use std::sync::Arc;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::transforms::{Filter, Transform};

#[derive(Clone)]
pub struct TransformBuilder<'a> {
    plex_client: &'a PlexClient,
    options: &'a PlexContext,
    transforms: Vec<Arc<dyn Transform + Send + Sync>>,
    filters: Vec<Arc<dyn Filter + Send + Sync>>,
}

impl<'a> TransformBuilder<'a> {
    pub fn new(plex_client: &'a PlexClient, options: &'a PlexContext) -> Self {
        Self {
            plex_client,
            options,
            transforms: Vec::new(),
            filters: Vec::new(),
        }
    }

    pub fn with_transform<T: Transform + Send + Sync + 'static>(
        mut self, transform: T,
    ) -> Self {
        self.transforms.push(Arc::new(transform));
        self
    }

    pub fn with_filter<T: Filter + Send + Sync + 'static>(
        mut self, filter: T,
    ) -> Self {
        self.filters.push(Arc::new(filter));
        self
    }

    pub async fn apply_to(&self, container: &mut MediaContainer) -> Result<()> {
        // Apply transformations to the whole container, maintaining async context.
        for transform in &self.transforms {
            transform
                .transform_mediacontainer(
                    container,
                    self.plex_client,
                    self.options,
                )
                .await?;
        }

        // Apply transformations and filters for each metadata item within the container
        let mut i = 0;
        while i < container.children_mut().len() {
            let item = &mut container.children_mut()[i];

            // Apply metadata-level transformations
            for transform in &self.transforms {
                transform
                    .transform_metadata(item, self.plex_client, self.options)
                    .await?;
            }

            // Determine if item should be included after filters are applied
            let include = self.filters.iter().all(|filter| {
                block_on(filter.filter_metadata(
                    item,
                    self.plex_client,
                    self.options,
                ))
            });

            if !include {
                container.children_mut().remove(i);
            } else {
                i += 1;
            }
        }

        // Update the container size based on the filtered and transformed children
        container.size = Some(container.children().len() as i64);

        Ok(())
    }
}
