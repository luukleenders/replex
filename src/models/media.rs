use std::fmt;

use crate::models::{SpecialBool, Stream};
use serde_aux::prelude::deserialize_number_from_string;

use replex_common::{struct_derives, struct_imports};

struct_imports!();

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[yaserde(attribute)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i64,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[yaserde(attribute, rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    //#[yaserde(attribute, rename = "aspectRatio")]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub aspect_ratio: Option<f64>,
    #[yaserde(attribute, rename = "audioChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i64>,
    #[yaserde(attribute, rename = "audioCodec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<String>,
    #[yaserde(attribute, rename = "videoCodec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    #[yaserde(attribute, rename = "videoResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_resolution: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[yaserde(attribute, rename = "partCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_count: Option<i32>,
    #[yaserde(attribute, rename = "channelArt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_art: Option<String>,
    #[yaserde(attribute, rename = "videoProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_profile: Option<String>,
    #[yaserde(attribute, rename = "videoFrameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_frame_rate: Option<String>,
    #[yaserde(attribute, rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[yaserde(attribute, rename = "optimizedForStreaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_for_streaming: Option<SpecialBool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[yaserde(rename = "Part", child)]
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "Part")]
    pub parts: Vec<MediaPart>,
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} - {}",
            self.video_resolution.clone().unwrap_or_default(),
            self.video_codec.clone().unwrap_or_default(),
            self.audio_codec.clone().unwrap_or_default()
        )
    }
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct MediaPart {
    #[yaserde(attribute)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i64,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[yaserde(attribute, rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[yaserde(attribute, rename = "videoProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_profile: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[yaserde(attribute, rename = "optimizedForStreaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_for_streaming: Option<SpecialBool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[yaserde(rename = "Stream", child)]
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "Stream")]
    pub streams: Vec<Stream>,
}
