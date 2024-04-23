use anyhow::Result;
use async_trait::async_trait;

use crate::models::{ClientHeroStyle, Image, MediaContainer, MetaData, Style};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::HeroArt;
use crate::utils::hero_meta;

use super::Transform;

pub struct MediaStyleTransform {
    pub style: Style,
}

#[async_trait]
impl Transform for MediaStyleTransform {
    async fn transform_mediacontainer(
        &self,
        item: &mut MediaContainer,
        _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        if self.style == Style::Hero {
            item.meta = Some(hero_meta());
        }

        Ok(())
    }

    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        plex_client: &PlexClient,
        options: &PlexContext,
    ) -> Result<()> {
        if self.style == Style::Hero {
            let mut guid = item.guid.clone().unwrap();
            let style_def = ClientHeroStyle::from_context(options);

            if guid.starts_with("plex://episode") && item.parent_guid.is_some()
            {
                guid = item.parent_guid.clone().unwrap();
            }

            if let Some(child_type) = &style_def.child_type {
                item.r#type = child_type.clone();
            }

            if let Some(cover_art) = HeroArt::get(plex_client, &guid).await? {
                item.images = vec![Image {
                    r#type: "coverArt".to_string(),
                    url: cover_art.clone(),
                    alt: Some(item.title.clone()),
                }];

                if style_def.cover_art_as_art {
                    item.art = Some(cover_art.clone());
                }

                if style_def.cover_art_as_thumb {
                    item.thumb = Some(cover_art);
                }
            }
        }

        Ok(())
    }
}
