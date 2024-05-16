type HyperResponse = hyper::Response<ResBody>;

use std::str;

use async_trait::async_trait;
use bincode::{Decode, Encode};
use bytes::Bytes;
use http_body_util::BodyExt;
use salvo::{
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        ResBody, Response, StatusError,
    },
    writing::Json,
    Error, Scribe,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;
use yaserde::{ser::to_string as to_xml_str, YaSerialize};
use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

use crate::config::Config;
use crate::models::{ContentType, Meta, MetaData, SpecialBool};
use crate::plex::client::PlexClient;
use crate::utils::sort_by_last_viewed;

/// NOTICE: Cant set yaserde on this? it will complain about a generic
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, YaDeserialize, YaSerialize,
)]
#[serde(rename_all = "camelCase")]
#[yaserde(root = "MediaContainer")]
pub struct WrappedMediaContainer {
    #[serde(default, rename = "MediaContainer")]
    pub media_container: MediaContainer,
    #[serde(skip_serializing, skip_deserializing)]
    pub content_type: ContentType,
}

// impl Default for WrappedMediaContainer {
//     fn default() -> Self { limit: None }
// }

impl WrappedMediaContainer {
    /// Returns a new empty WrappedMediaContainer with the given content type.
    pub fn empty(content_type: ContentType) -> Self {
        Self {
            content_type,
            media_container: MediaContainer {
                size: Some(0),
                identifier: Some("com.plexapp.plugins.library".to_string()),
                ..MediaContainer::default()
            },
        }
    }

    pub fn is_hub(&self) -> bool {
        !self.media_container.hub.is_empty()
    }

    pub fn is_section_hub(&self) -> bool {
        self.is_hub() && self.media_container.library_section_id.is_some()
    }
}

impl Scribe for WrappedMediaContainer {
    #[inline]
    fn render(self, res: &mut Response) {
        match &self.content_type {
            ContentType::Json => Json(self).render(res),
            ContentType::Xml => Xml(self.media_container).render(res),
        }
    }
}

pub struct Xml<T>(pub T);

#[async_trait]
impl<T> Scribe for Xml<T>
where
    T: YaSerialize + Send,
{
    #[inline]
    fn render(self, res: &mut Response) {
        match to_xml_str(&self.0) {
            Ok(bytes) => {
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("text/xml; charset=utf-8"),
                );
                res.write_body(bytes).ok();
            }
            Err(e) => {
                tracing::error!(error = ?e, "Xml write error");
                res.render(StatusError::internal_server_error());
            }
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    YaDeserialize,
    YaSerialize,
    Default,
    Encode,
    Decode,
)]
#[serde_as]
#[serde(rename_all = "camelCase")]
#[yaserde(root = "MediaContainer")]
pub struct MediaContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute)]
    pub size: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute, rename = "totalSize")]
    pub total_size: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute)]
    pub offset: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute, rename = "allowSync")]
    pub allow_sync: Option<SpecialBool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute)]
    pub identifier: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute)]
    pub parent_title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute)]
    pub title_2: Option<String>,

    #[serde(
        default,
        rename = "librarySectionID",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_number_from_string"
    )]
    #[yaserde(attribute, rename = "librarySectionID")]
    pub library_section_id: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute, rename = "librarySectionTitle")]
    pub library_section_title: Option<String>,

    #[serde(
        rename = "librarySectionUUID",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "librarySectionUUID")]
    pub library_section_uuid: Option<String>,

    #[serde(default, rename = "Hub", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Hub")]
    pub hub: Vec<MetaData>,

    #[serde(
        default,
        rename = "Metadata",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[yaserde(rename = "Metadata")]
    pub metadata: Vec<MetaData>,

    #[serde(default, rename = "Video", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Video")]
    pub video: Vec<MetaData>,

    #[serde(
        default,
        rename = "Directory",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[yaserde(rename = "Directory")]
    pub directory: Vec<MetaData>,

    #[serde(
        default,
        rename = "playQueueID",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueID")]
    pub play_queue_id: Option<i64>,

    #[serde(
        default,
        rename = "playQueueSelectedItemID",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueSelectedItemID")]
    pub play_queue_selected_item_id: Option<i64>,

    #[serde(
        default,
        rename = "playQueueSelectedItemOffset",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueSelectedItemOffset")]
    pub play_queue_selected_item_offset: Option<i32>,

    #[serde(
        default,
        rename = "playQueueSelectedMetadataItemID",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueSelectedMetadataItemID")]
    pub play_queue_selected_metadata_item_id: Option<String>,

    #[serde(
        default,
        rename = "playQueueShuffled",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueShuffled")]
    pub play_queue_shuffled: Option<bool>,

    #[serde(
        default,
        rename = "playQueueSourceURI",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueSourceURI")]
    pub play_queue_source_uri: Option<String>,

    #[serde(
        default,
        rename = "playQueueTotalCount",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueTotalCount")]
    pub play_queue_total_count: Option<i32>,

    #[serde(
        default,
        rename = "playQueueVersion",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "playQueueVersion")]
    pub play_queue_version: Option<i32>,

    #[serde(
        default,
        rename = "mediaTagPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "mediaTagPrefix")]
    pub media_tag_prefix: Option<String>,

    #[serde(
        default,
        rename = "mediaTagVersion",
        deserialize_with = "option_number_from_string",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "mediaTagVersion")]
    pub media_tag_version: Option<i64>,

    #[serde(
        default,
        rename = "directPlayDecisionCode",
        deserialize_with = "option_number_from_string",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "directPlayDecisionCode")]
    pub direct_play_decision_code: Option<i64>,

    #[serde(
        default,
        rename = "directPlayDecisionText",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "directPlayDecisionText")]
    pub direct_play_decision_text: Option<String>,

    #[serde(
        default,
        rename = "generalDecisionCode",
        deserialize_with = "option_number_from_string",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "generalDecisionCode")]
    pub general_decision_code: Option<i64>,

    #[serde(
        default,
        rename = "generalDecisionText",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "generalDecisionText")]
    pub general_decision_text: Option<String>,

    #[serde(
        default,
        rename = "resourceSession",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "resourceSession")]
    pub resource_session: Option<String>,

    #[serde(
        default,
        rename = "transcodeDecisionCode",
        deserialize_with = "option_number_from_string",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "transcodeDecisionCode")]
    pub transcode_decision_code: Option<i64>,

    #[serde(
        default,
        rename = "transcodeDecisionText",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute, rename = "transcodeDecisionText")]
    pub transcode_decision_text: Option<String>,

    #[serde(default, rename = "Meta", skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute, rename = "Meta")]
    pub meta: Option<Meta>,
}

pub(crate) fn option_number_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_aux::prelude::deserialize_option_number_from_string::<i64, D>(
        deserializer,
    ) {
        Ok(r) => Ok(r),
        Err(_) => Ok(None),
    }
}

impl MediaContainer {
    pub fn wrap(self, content_type: ContentType) -> WrappedMediaContainer {
        WrappedMediaContainer {
            media_container: self,
            content_type,
        }
    }

    pub async fn better_on_deck(
        &mut self,
        collection_title: &str,
        plex_client: &PlexClient,
    ) {
        let config = Config::load();

        if config.better_on_deck {
            if let Some(in_progress) = &config.in_progress {
                if collection_title == in_progress {
                    sort_by_last_viewed(plex_client, self.children_mut()).await;
                }
            }

            if let Some(next_up) = &config.next_up {
                if collection_title == next_up {
                    sort_by_last_viewed(plex_client, self.children_mut()).await;
                }
            }
        }
    }

    pub async fn from_reqwest_response(
        res: reqwest::Response,
    ) -> Result<Self, Error> {
        let bytes = res.bytes().await.unwrap();

        Self::from_bytes(bytes).await
    }

    pub async fn from_hyper_response(
        res: HyperResponse,
    ) -> Result<Self, Error> {
        let bytes = res.into_body().collect().await.unwrap().to_bytes();

        Self::from_bytes(bytes).await
    }

    pub async fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        // Attempt to convert bytes to a UTF-8 string
        match str::from_utf8(&bytes) {
            Ok(json_str) => {
                // Proceed with deserialization
                let deserializer = &mut serde_json::Deserializer::from_reader(
                    json_str.as_bytes(),
                );
                let result: WrappedMediaContainer =
                    serde_path_to_error::deserialize(deserializer)
                        .map_err(Error::other)?;

                Ok(result.media_container)
            }
            Err(e) => {
                // Log an error if the bytes cannot be converted to a string
                tracing::error!(
                    "Failed to convert bytes to UTF-8 string: {}",
                    e
                );
                Err(Error::other(e))
            }
        }
    }

    pub fn is_hub(&self) -> bool {
        !self.hub.is_empty()
    }

    pub fn exclude_watched(&self) -> bool {
        let _config = Config::load();

        // if config.exclude_watched {
        //     return true;
        // }

        if let Some(first_meta) = self.metadata.first() {
            return first_meta.has_label("REPLEX_EXCLUDE_WATCHED".to_string());
        }

        false
    }

    pub fn set_type(&mut self, value: String) {
        for hub in &mut self.hub {
            hub.r#type = value.clone();
        }
    }

    pub fn set_children(&mut self, value: Vec<MetaData>) {
        let len: i64 = value.len().try_into().unwrap();
        if !self.metadata.is_empty() {
            self.metadata = value;
        } else if !self.hub.is_empty() {
            self.hub = value;
        } else if !self.video.is_empty() {
            self.video = value;
        } else if !self.directory.is_empty() {
            self.directory = value;
        };
        self.size = Some(len);
    }

    pub fn set_children_mut(&mut self, value: &mut Vec<MetaData>) {
        let len: i64 = value.len().try_into().unwrap();
        if !self.metadata.is_empty() {
            self.metadata = value.to_owned();
        } else if !self.hub.is_empty() {
            self.hub = value.to_owned();
        } else if !self.video.is_empty() {
            self.video = value.to_owned();
        } else if !self.directory.is_empty() {
            self.directory = value.to_owned();
        };
        self.size = Some(len);
    }

    pub fn children_mut(&mut self) -> &mut Vec<MetaData> {
        if !self.metadata.is_empty() {
            return &mut self.metadata;
        } else if !self.hub.is_empty() {
            return &mut self.hub;
        } else if !self.video.is_empty() {
            return &mut self.video;
        } else if !self.directory.is_empty() {
            return &mut self.directory;
        };
        &mut self.metadata
    }

    pub fn children(&mut self) -> Vec<MetaData> {
        if !self.metadata.is_empty() {
            return self.metadata.clone();
        } else if !self.hub.is_empty() {
            return self.hub.clone();
        } else if !self.video.is_empty() {
            return self.video.clone();
        } else if !self.directory.is_empty() {
            return self.directory.clone();
        };
        vec![]
    }
}
