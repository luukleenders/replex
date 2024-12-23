use anyhow::Result;
use async_trait::async_trait;

use crate::models::{ClientHeroStyle, Image, MediaContainer, MetaData, Style};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::Images;
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
        options: &PlexContext,
    ) -> Result<()> {
        if self.style == Style::Hero {
            item.meta = Some(hero_meta(options.platform.clone()));
        }

        Ok(())
    }

    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: &PlexClient,
        options: &PlexContext,
    ) -> Result<()> {
        if self.style == Style::Hero {
            // let config = Config::load();
            let mut guid = item.guid.clone().unwrap();
            let style_def = ClientHeroStyle::from_context(options);

            if guid.starts_with("plex://episode") && item.parent_guid.is_some()
            {
                guid = item.parent_guid.clone().unwrap();
            }

            if let Some(child_type) = &style_def.child_type {
                item.r#type = child_type.clone();
            }

            guid = guid.replace("plex://", "");
            let _host = options.host.clone().unwrap();
            let cover_art = Some(format!("{}://{}/replex/image/hero/{}/{}",
                 options.forwarded_proto.clone().unwrap_or_else(|| "http".to_string()),
                 match options.forwarded_host.clone() {
                     Some(v) => v,
                     None => options.host.clone().unwrap()
                 },
                 guid,
                 options.token.clone().unwrap()
            ));

            let images = Images::get(_plex_client, &guid).await?;

            if let Some(cover_art) = cover_art {
                if let Some(mut images) = images {
                    for image in &mut images {
                        if image.r#type == "coverArt" {
                            image.url = cover_art.clone();
                            break;
                        }
                    }
                    item.images = images;
                } else {
                    item.images = vec![Image {
                        r#type: "coverArt".to_string(),
                        url: cover_art.clone(),
                        alt: Some(item.title.clone()),
                    }];
                }


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
