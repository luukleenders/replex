use crate::deserializers::{
    bool_from_int, deserialize_comma_separated_string,
    deserialize_screen_resolution,
};
use crate::models::{Platform, Resolution};
use salvo::macros::Extractible;
use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

#[derive(Serialize, Deserialize, Debug, Extractible, Default, Clone, Encode, Decode)]
#[salvo(extract(
    default_source(from = "query"),
    default_source(from = "header"),
    rename_all = "camelCase"
))]
pub struct PlexContext {
    #[serde(default, deserialize_with = "deserialize_comma_separated_string")]
    #[salvo(extract(rename = "contentDirectoryID"))]
    pub content_directory_id: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string")]
    #[salvo(extract(rename = "pinnedContentDirectoryID"))]
    pub pinned_content_directory_id: Option<Vec<String>>,
    #[serde(default)]
    #[salvo(extract(rename = "X-Plex-Platform"))]
    pub platform: Platform,
    #[serde(default, deserialize_with = "deserialize_screen_resolution")]
    #[salvo(extract(rename = "X-Plex-Device-Screen-Resolution"))]
    pub screen_resolution: Vec<Resolution>,
    #[salvo(extract(rename = "X-Plex-Device-Screen-Resolution"))]
    pub screen_resolution_original: Option<String>,
    #[salvo(extract(rename = "X-Plex-Client-Capabilities", alias = "x-plex-client-capabilities"))]
    pub client_capabilities: Option<String>,
    #[salvo(extract(rename = "X-Plex-Product"))]
    pub product: Option<String>,
    #[salvo(extract(rename = "X-Plex-Version"))]
    pub version: Option<String>,
    pub count: Option<i32>,
    #[salvo(extract(rename = "X-Plex-Client-Identifier", alias = "x-plex-client-identifier"))]
    pub client_identifier: Option<String>,
    #[salvo(extract(rename = "X-Plex-Session-Id", alias = "x-plex-session-id"))]
    pub session_id: Option<String>,
    #[salvo(extract(rename = "X-Plex-Session-Identifier", alias = "x-plex-session-identifier"))]
    pub session_identifier: Option<String>,
    #[salvo(extract(rename = "X-Plex-Playback-Session-Id", alias = "x-plex-playback-session-id"))]
    pub playback_session_id: Option<String>,
    #[salvo(extract(rename = "X-Plex-Playback-Id"))]
    pub playback_id: Option<String>,
    #[salvo(extract(rename = "X-Plex-Token"))]
    pub token: Option<String>,
    #[salvo(extract(rename = "X-Plex-Platform-Version"))]
    pub platform_version: Option<String>,
    #[salvo(extract(rename = "X-Plex-Features"))]
    pub features: Option<String>,
    #[salvo(extract(rename = "X-Plex-Model"))]
    pub model: Option<String>,
    #[salvo(extract(rename = "X-Plex-Device"))]
    pub device: Option<String>,
    #[salvo(extract(rename = "X-Plex-Device-Name"))]
    pub device_name: Option<String>,
    #[salvo(extract(rename = "X-Plex-Drm"))]
    pub drm: Option<String>,
    #[salvo(extract(rename = "X-Plex-Text-Format"))]
    pub text_format: Option<String>,
    #[salvo(extract(rename = "X-Plex-Provider-Version"))]
    pub provider_version: Option<String>,
    #[salvo(extract(rename = "X-Plex-Container-Size"))]
    pub container_size: Option<i32>,
    #[salvo(extract(rename = "X-Plex-Container-Start"))]
    pub container_start: Option<i32>,
    #[salvo(extract(rename = "X-Plex-Username"))]
    pub username: Option<String>,
    #[salvo(extract(rename = "x-plex-http-pipeline"))]
    pub http_pipeline: Option<String>,
    #[serde(default, deserialize_with = "bool_from_int")]
    #[salvo(extract(rename = "includeCollections"))]
    pub include_collections: bool,
    #[serde(default, deserialize_with = "bool_from_int")]
    #[salvo(extract(rename = "includeAdvanced"))]
    pub include_advanced: bool,
    #[salvo(extract(rename = "X-Forwarded-For", alias = "X-Real-Ip"))]
    pub forwarded_for: Option<String>,
    #[salvo(extract(rename = "X-Forwarded-Proto", alias = "x-forwarded-proto"))]
    pub forwarded_proto: Option<String>,
    #[salvo(extract(rename = "X-Forwarded-Host", alias = "x-forwarded-host"))]
    pub forwarded_host: Option<String>,
    #[salvo(extract(rename = "X-Forwarded-Port", alias = "x-forwarded-port"))]
    pub forwarded_port: Option<String>,
    #[serde(default, deserialize_with = "bool_from_int")]
    #[salvo(extract(rename = "excludeAllLeaves"))]
    pub exclude_all_leaves: bool,
    #[salvo(extract(rename = "host"))]
    pub host: Option<String>,
    // photo transcode
    pub size: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub quality: Option<i32>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Encode, Decode)]
pub struct PlexContextProduct {
    #[serde(default, rename(deserialize = "x-plex-product"))]
    pub product: Option<String>,
}
