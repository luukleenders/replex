use anyhow::Result;
use async_trait::async_trait;
use futures::stream::FuturesOrdered;
use futures_util::StreamExt;

use crate::models::{ClientHeroStyle, MetaData, Style};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::utils::hero_meta;

use super::{MediaStyleTransform, Transform};

#[derive(Default, Debug)]
pub struct HubStyleTransform {
    pub is_home: bool, // use clip instead of hero for android
}

#[async_trait]
impl Transform for HubStyleTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        plex_client: &PlexClient,
        options: &PlexContext,
    ) -> Result<()> {
        // TODO: Check why tries to load non existing collection? my guess is no access
        if !item.is_hub() || !item.is_hero(plex_client).await.unwrap_or(false) {
            return Ok(());
        }

        let style = ClientHeroStyle::from_context(options);
        let children = item.children();
        let child_type = style.child_type;
        let mut futures = FuturesOrdered::new();

        item.style = style.style;
        item.r#type = style.r#type;
        item.meta = Some(hero_meta());
        item.placeholder = Some(true);

        for mut child in children {
            if let Some(ref child_type) = child_type {
                child.r#type = child_type.clone();
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
        // let filtered_children = children.into_iter().flatten().collect();

        item.set_children(children);

        return Ok(());
    }
}
