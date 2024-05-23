use anyhow::Result;
use async_trait::async_trait;
use replex_common::{struct_derives, struct_imports};
use salvo::macros::Extractible;
use serde_aux::prelude::deserialize_number_from_string;

use crate::plex::models::PlexContext;

use super::{DeviceType, MediaContainer, Platform};

struct_imports!();

#[struct_derives]
pub struct ReplexOptions {
    pub limit: Option<i32>,
    pub platform: Option<String>,
    #[serde(default)]
    pub include_watched: bool,
}

#[struct_derives()]
#[derive(Extractible)]
#[salvo(extract(
    default_source(from = "query"),
    default_source(from = "header"),
    rename_all = "camelCase"
))]
pub struct Resolution {
    pub height: i64,
    pub width: i64,
}

pub struct TranscodingStatus {
    pub is_transcoding: bool,
    pub decision_result: MediaContainer,
}

#[async_trait]
pub trait FromResponse<T>: Sized {
    async fn from_response(resp: T) -> Result<Self>;
}

#[struct_derives()]
pub struct Guid {
    #[yaserde(attribute)]
    id: String,
}

#[struct_derives()]
pub struct Tag {
    #[yaserde(attribute)]
    tag: String,
}

#[struct_derives()]
pub struct Image {
    #[serde(default)]
    #[yaserde(attribute)]
    pub alt: Option<String>,

    #[serde(default, rename = "type")]
    #[yaserde(attribute, rename = "type")]
    pub r#type: String,

    #[serde(default)]
    #[yaserde(attribute)]
    pub url: String,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct Label {
    #[yaserde(attribute)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: i64,
    #[yaserde(attribute)]
    pub tag: String,
    #[yaserde(attribute)]
    filter: String,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "Image", default, skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Image", default, child)]
    pub images: Vec<Image>,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct DisplayField {
    #[yaserde(attribute, rename = "type")]
    pub r#type: Option<String>,
    pub fields: Vec<String>,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct MetaType {
    #[yaserde(attribute, rename = "type")]
    pub r#type: Option<String>,

    #[yaserde(attribute)]
    pub active: Option<bool>,

    #[yaserde(attribute)]
    pub title: Option<String>,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct DisplayImage {
    #[yaserde(attribute, rename = "type")]
    pub r#type: Option<String>,

    #[serde(rename = "imageType")]
    #[yaserde(attribute, rename = "imageType")]
    pub image_type: Option<String>,
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(default, rename = "DisplayFields")]
    pub display_fields: Vec<DisplayField>,

    #[serde(default, rename = "DisplayImage")]
    pub display_images: Vec<DisplayImage>,

    #[serde(default)]
    #[yaserde(attribute, rename = "type")]
    pub r#type: Option<MetaType>,
    // #[yaserde(attribute)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub style: Option<String>,
}

#[derive(Debug)]
pub struct ClientHeroStyle {
    pub _enabled: bool,
    pub r#type: String,
    pub style: Option<String>,
    pub child_type: Option<String>,
    pub cover_art_as_thumb: bool, // if we should return the coverart in the thumb field
    pub cover_art_as_art: bool,   // if we should return the coverart in the art field
}

impl Default for ClientHeroStyle {
    fn default() -> Self {
        Self {
            _enabled: true,
            style: Some("hero".to_string()),
            r#type: "mixed".to_string(),
            child_type: None,
            cover_art_as_thumb: false,
            cover_art_as_art: true,
        }
    }
}

impl ClientHeroStyle {
    pub fn from_context(context: &PlexContext) -> Self {
        // pub fn android(product: String, platform_version: String) -> Self {
        let product = context.product.clone().unwrap_or_default();
        let device_type = DeviceType::from_product(product);
        let platform = context.platform.clone();
        // let platform_version = context.platform_version.clone().unwrap_or_default();

        match platform {
            Platform::Android => {
                match device_type {
                    DeviceType::Tv => {
                        Self {
                            style: Some("hero".to_string()),
                            // clip wil make the item info disappear on TV
                            r#type: "mixed".to_string(),
                            // using clip makes it load thumbs instead of art as cover art. So we don't have to touch the background
                            child_type: Some("clip".to_string()),
                            cover_art_as_art: false,
                            cover_art_as_thumb: true,
                            ..ClientHeroStyle::default()
                        }
                    }
                    _ => Self {
                        style: None,
                        r#type: "clip".to_string(),
                        child_type: Some("clip".to_string()),
                        cover_art_as_art: true,
                        ..ClientHeroStyle::default()
                    },
                }
            }
            Platform::Roku => ClientHeroStyle::roku(),
            Platform::Ios => ClientHeroStyle::ios_style(),
            Platform::TvOS => ClientHeroStyle::tvos_style(),
            _ => ClientHeroStyle::default(),
            // _ => {
            //     if product.starts_with("Plex HTPC") {
            //         ClientHeroStyle::htpc_style()
            //     } else {
            //         match product.to_lowercase().as_ref() {
            //             "plex for lg" => ClientHeroStyle::htpc_style(),
            //             "plex for xbox" => ClientHeroStyle::htpc_style(),
            //             "plex for ps4" => ClientHeroStyle::htpc_style(),
            //             "plex for ps5" => ClientHeroStyle::htpc_style(),
            //             "plex for ios" => ClientHeroStyle::ios_style(),
            //             _ => ClientHeroStyle::default(),
            //         }
            //     }
            // }
        }
    }

    pub fn roku() -> Self {
        Self {
            style: Some("hero".to_string()),
            ..ClientHeroStyle::default()
        }
    }

    pub fn htpc_style() -> Self {
        Self {
            ..ClientHeroStyle::default()
        }
    }

    pub fn ios_style() -> Self {
        Self {
            cover_art_as_art: true,
            cover_art_as_thumb: false, // ios doesnt load the subview as hero.
            ..ClientHeroStyle::default()
        }
    }

    pub fn tvos_style() -> Self {
        Self {
            cover_art_as_art: true,
            cover_art_as_thumb: false, // ios doesnt load the subview as hero.
            ..ClientHeroStyle::default()
        }
    }

    // pub fn for_client(platform: Platform, product: String, platform_version: String) -> Self {
    //     match platform {
    //         Platform::Android => PlatformHeroStyle::android(product, platform_version),
    //         Platform::Roku => PlatformHeroStyle::roku(product),
    //         _ => {
    //             if product.starts_with("Plex HTPC") {
    //               ClientHeroStyle::htpc_style()
    //             } else {
    //                 match product.to_lowercase().as_ref() {
    //                     "plex for lg" => ClientHeroStyle::htpc_style(),
    //                     "plex for xbox" => ClientHeroStyle::htpc_style(),
    //                     "plex for ps4" => ClientHeroStyle::htpc_style(),
    //                     "plex for ps5" => ClientHeroStyle::htpc_style(),
    //                     "plex for ios" => ClientHeroStyle::ios_style(),
    //                     _ => ClientHeroStyle::default(),
    //                 }
    //             }
    //         }
    //     }
    // }
}
