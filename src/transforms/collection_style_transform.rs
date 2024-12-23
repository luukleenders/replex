use anyhow::Result;
use async_trait::async_trait;
use futures_util::{stream::FuturesOrdered, StreamExt};

use crate::models::{ClientHeroStyle, MediaContainer, MetaData, Style};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::Collection;
use crate::transforms::Transform;
use crate::utils::hero_meta;

use super::MediaStyleTransform;

#[derive(Default, Debug)]
pub struct CollectionStyleTransform {
    pub collection_ids: Vec<i64>,
    pub is_hub: bool,
}

#[async_trait]
impl Transform for CollectionStyleTransform {
    async fn transform_mediacontainer(
        &self,
        container: &mut MediaContainer,
        plex_client: &PlexClient,
        options: &PlexContext,
    ) -> Result<()> {
        let mut collection =
            Collection::get(plex_client, self.collection_ids[0])
                .await
                .unwrap();

        let is_hero = collection
            .children()
            .first()
            .unwrap()
            .has_label("REPLEXHERO".to_string());

        if is_hero {
            let children = container.children();
            let style = ClientHeroStyle::from_context(options);
            let mut futures = FuturesOrdered::new();

            container.meta = Some(hero_meta(options.platform.clone()));


            for mut child in children {
                if let Some(ref child_type) = style.child_type {
                    child.r#type = child_type.clone();
                } else {
                    child.r#type = "clip".to_string();
                }

                futures.push_back(async move {
                    let transform = MediaStyleTransform { style: Style::Hero };
                    transform
                        .transform_metadata(&mut child, plex_client, options)
                        .await
                        .expect("Failed to transform metadata");
                    child
                });
            }

            let children: Vec<MetaData> = futures.collect().await;
            container.set_children(children);
        }

        Ok(())
    }
}
