use replex_common::{struct_derives, struct_imports};
use serde_aux::prelude::deserialize_number_from_string;

struct_imports!();

#[struct_derives()]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    #[yaserde(attribute)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i64,
    #[yaserde(attribute, rename = "streamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[yaserde(attribute, rename = "languageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_tag: Option<String>,
    #[yaserde(attribute, rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[yaserde(attribute, rename = "DOVIBLCompatID")]
    #[serde(rename = "DOVIBLCompatID", skip_serializing_if = "Option::is_none")]
    pub doviblcompat_id: Option<i64>,
    #[yaserde(attribute, rename = "DOVIBLPresent")]
    #[serde(rename = "DOVIBLPresent", skip_serializing_if = "Option::is_none")]
    pub doviblpresent: Option<bool>,
    #[yaserde(attribute, rename = "DOVIELPresent")]
    #[serde(rename = "DOVIELPresent")]
    pub dovielpresent: Option<bool>,
    #[yaserde(attribute, rename = "DOVILevel")]
    #[serde(rename = "DOVILevel", skip_serializing_if = "Option::is_none")]
    pub dovilevel: Option<i64>,
    #[yaserde(attribute, rename = "DOVIPresent")]
    #[serde(rename = "DOVIPresent", skip_serializing_if = "Option::is_none")]
    pub dovipresent: Option<bool>,
    #[yaserde(attribute, rename = "DOVIProfile")]
    #[serde(rename = "DOVIProfile", skip_serializing_if = "Option::is_none")]
    pub doviprofile: Option<i64>,
    #[yaserde(attribute, rename = "DOVIRPUPresent")]
    #[serde(rename = "DOVIRPUPresent", skip_serializing_if = "Option::is_none")]
    pub dovirpupresent: Option<bool>,
    #[yaserde(attribute, rename = "DOVIVersion")]
    #[serde(rename = "DOVIVersion", skip_serializing_if = "Option::is_none")]
    pub doviversion: Option<String>,
    #[yaserde(attribute, rename = "bitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    #[yaserde(attribute, rename = "chromaLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chroma_location: Option<String>,
    #[yaserde(attribute, rename = "chromaSubsampling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chroma_subsampling: Option<String>,
    #[yaserde(attribute, rename = "codeHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_height: Option<i64>,
    #[yaserde(attribute, rename = "codeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_width: Option<i64>,
    #[yaserde(attribute, rename = "colorPrimaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_primaries: Option<String>,
    #[yaserde(attribute, rename = "colorRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_range: Option<String>,
    #[yaserde(attribute, rename = "colorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[yaserde(attribute, rename = "colorTrc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_trc: Option<String>,
    #[yaserde(attribute, rename = "frameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<bool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[yaserde(attribute, rename = "refFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_frames: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[yaserde(attribute, rename = "displayTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    #[yaserde(attribute, rename = "extendedDisplaytitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_display_title: Option<String>,
    #[yaserde(attribute, rename = "hasScalingMatrix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_scaling_matrix: Option<bool>,
    #[yaserde(attribute, rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    #[yaserde(attribute, rename = "audioChannelLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_layout: Option<String>,
    #[yaserde(attribute, rename = "samplingRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<i64>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced: Option<bool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[yaserde(attribute, rename = "hearingImpaired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearing_impaired: Option<bool>,
    #[yaserde(attribute)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
}
