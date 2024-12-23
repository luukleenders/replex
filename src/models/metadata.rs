use crate::config::*;
use crate::models::*;
use crate::plex::client::PlexClient;
use crate::utils::*;
use anyhow::Result;
use serde_with::serde_as;

use crate::deserializers::{option_number_from_string, option_string_from_number};
use crate::plex::traits::Collection;

use replex_common::{struct_derives, struct_imports};

struct_imports!();

fn default_label() -> Vec<Label> {
    vec![Label::default()]
}

fn default_image() -> Vec<Image> {
    vec![Image::default()]
}

fn default_media() -> Vec<Media> {
    vec![Media::default()]
}

fn default_guid() -> Vec<Guid> {
    vec![Guid::default()]
}

fn default_tag() -> Vec<Tag> {
    vec![Tag::default()]
}

#[struct_derives()]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MetaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "ratingKey")]
    pub rating_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub guid: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "primaryGuid")]
    pub primary_guid: Option<String>,

    #[serde(default)]
    #[yaserde(attribute = true)]
    pub title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub slug: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub tagline: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub thumb: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub theme: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub composite: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "playlistType")]
    pub playlist_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub banner: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub icon: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub view_group: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "addedAt")]
    pub added_at: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "updatedAt")]
    pub updated_at: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "lastViewedAt")]
    pub last_viewed_at: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "includedAt")]
    pub included_at: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub duration: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub view_mode: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub art: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub index: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub subtype: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub studio: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "contentRating")]
    pub content_rating: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub rating: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "audienceRating")]
    pub audience_rating: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "viewOffset")]
    pub view_offset: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "primaryExtraKey")]
    pub primary_extra_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "chapterSource")]
    pub chapter_source: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "ratingImage")]
    pub rating_image: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "audienceRatingImage")]
    pub audiance_rating_image: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentYear")]
    pub parent_year: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentIndex")]
    pub parent_index: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentGuid")]
    pub parent_guid: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentStudio")]
    pub parent_studio: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentKey")]
    pub parent_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentRatingKey")]
    pub parent_rating_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentTitle")]
    pub parent_title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentArt")]
    pub parent_art: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "parentThumb")]
    pub parent_thumb: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentRatingKey")]
    pub grandparent_rating_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentKey")]
    pub grandparent_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentGuid")]
    pub grandparent_guid: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentTitle")]
    pub grandparent_title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentThumb")]
    pub grandparent_thumb: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "grandparentArt")]
    pub grandparent_art: Option<String>,

    #[serde(
        default,
        rename = "librarySectionID",
        deserialize_with = "option_number_from_string",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute = true, rename = "librarySectionID")]
    pub library_section_id: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "librarySectionTitle")]
    pub library_section_title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "librarySectionKey")]
    pub library_section_key: Option<String>,

    #[serde(default)]
    #[yaserde(attribute = true, rename = "type")]
    pub r#type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub summary: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub year: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub promoted: Option<SpecialBool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "skipDetails")]
    pub skip_details: Option<SpecialBool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub placeholder: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub context: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "hubKey")]
    pub hub_key: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "hubIdentifier")]
    pub hub_identifier: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub size: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub more: Option<SpecialBool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true)]
    pub style: Option<String>,

    #[serde(default, rename = "Meta", skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "Meta")]
    pub meta: Option<Meta>,

    #[serde(default, rename = "Metadata", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Metadata")]
    pub metadata: Vec<MetaData>,

    #[serde(default, rename = "Directory", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Directory")]
    pub directory: Vec<MetaData>, // only avaiable in XML

    #[serde(default, rename = "Video", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(rename = "Video")]
    pub video: Vec<MetaData>, // again only xml, but its the same as directory and metadata

    #[serde(
        default,
        deserialize_with = "option_string_from_number",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute = true, rename = "childCount")]
    pub child_count: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "skipChildren")]
    pub skip_children: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "leafCount")]
    pub leaf_count: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "viewedLeafCount")]
    pub viewed_leaf_count: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "viewCount")]
    pub view_count: Option<i32>,

    #[serde(default, rename = "Label", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_label", rename = "Label")]
    pub labels: Vec<Label>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "originallyAvailableAt")]
    pub originally_available_at: Option<String>,

    #[serde(default, rename = "Media", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_media", rename = "Media")]
    pub media: Vec<Media>,

    #[serde(default, rename = "Guid", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_guid", rename = "Guid")]
    pub guids: Vec<Guid>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "State")]
    pub user_state: Option<SpecialBool>,

    #[serde(default, rename = "Image", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_image", rename = "Image")]
    pub images: Vec<Image>,

    #[serde(default, rename = "Context", skip_serializing_if = "Option::is_none")]
    #[yaserde(rename = "Context")]
    pub context_images: Option<Context>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[yaserde(attribute = true, rename = "extraType")]
    pub extra_type: Option<i32>, // actually a bool but plex does 0 and 1

    #[serde(
        default,
        rename = "playQueueItemID",
        skip_serializing_if = "Option::is_none"
    )]
    #[yaserde(attribute = true, rename = "playQueueItemID")]
    pub play_queue_item_id: Option<i64>,

    #[serde(default, rename = "Collection", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_tag", rename = "Collection")]
    pub collections: Vec<Tag>,

    #[serde(default, rename = "Country", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_tag", rename = "Country")]
    pub countries: Vec<Tag>,

    #[serde(default, rename = "Director", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_tag", rename = "Director")]
    pub directors: Vec<Tag>,

    #[serde(default, rename = "Genre", skip_serializing_if = "Vec::is_empty")]
    #[yaserde(default = "default_tag", rename = "Genre")]
    pub genres: Vec<Tag>,
}

impl MetaData {
    pub async fn better_on_deck(&mut self, plex_client: &PlexClient) {
        let config = Config::load();

        if config.better_on_deck.enabled {
            if let Some(in_progress) = &config.better_on_deck.in_progress {
                if &self.title == in_progress {
                    sort_by_last_viewed(plex_client, self.children_mut()).await;
                }
            }

            if let Some(next_up) = &config.better_on_deck.next_up {
                if &self.title == next_up {
                    sort_by_last_viewed(plex_client, self.children_mut()).await;
                }
            }
        }
    }

    pub fn has_label(&self, name: String) -> bool {
        for label in &self.labels {
            if label.tag.to_lowercase() == name.to_lowercase() {
                return true;
            }
        }
        false
    }

    /// if this hub should be hero style
    pub async fn is_hero(&self, plex_client: &PlexClient) -> Result<bool> {
        if !self.is_hub() {
            return Ok(false);
        }

        let config = Config::load();

        // Check if the hub identifier matches any of the hero row identifiers.
        if let Some(hero_rows) = &config.hero_rows {
            if let Some(hub_id) = &self.hub_identifier {
                if hero_rows
                    .iter()
                    .any(|row| !row.is_empty() && hub_id.contains(row))
                {
                    return Ok(true);
                }
            }
        }

        // Further checks for collection hubs, if necessary.
        if self.is_collection_hub() {
            let collection_id = get_collection_id_from_hub(self);
            let collection = Collection::get(plex_client, collection_id).await?;

            // Check if the first child of the collection details has the "REPLEXHERO" label.
            if let Some(collection) = collection.metadata.first() {
                return Ok(collection.has_label("REPLEXHERO".to_string()));
            }
        }

        Ok(false)
    }

    pub fn is_watched(&self) -> bool {
        let view_count = self.view_count.clone();
        let leaf_count = self.leaf_count.clone();
        let viewed_leaf_count = self.viewed_leaf_count.clone();

        if view_count.is_some() && view_count.unwrap() > 0 {
            if leaf_count.is_none() && viewed_leaf_count.is_none() {
                return true;
            }
        }

        if let (Some(leaf_count), Some(viewed_leaf_count)) = (leaf_count, viewed_leaf_count) {
            if leaf_count == viewed_leaf_count {
                return true;
            }
        }

        false
    }

    pub async fn exclude_watched(&self, plex_client: &PlexClient) -> Result<bool> {
        if !self.is_collection_hub() {
            return Ok(false);
        }

        let config = Config::load();

        if config.exclude_watched.all {
            return Ok(true);
        }

        let collection = Collection::get(plex_client, get_collection_id_from_hub(self)).await?;

        let has_excluded_label = collection
            .metadata
            .first()
            .unwrap()
            .has_label("REPLEX_EXCLUDE_WATCHED".to_string());

        let is_config_excluded = config
            .exclude_watched
            .collections
            .as_ref()
            .map_or(false, |collections| {
                collections.contains(&collection.metadata.first().unwrap().title)
            });

        Ok(has_excluded_label || is_config_excluded)
    }

    pub fn get_type(&self) -> String {
        if self.is_hub() {
            return "hub".to_string();
        }
        if self.is_media() {
            return "media".to_string();
        }

        "unknown".to_string()
    }

    pub fn is_hub(&self) -> bool {
        self.hub_identifier.is_some()
    }

    pub fn is_media(&self) -> bool {
        !self.is_hub() && (self.r#type == "movie" || self.r#type == "show")
    }

    pub fn is_playlist(&self) -> bool {
        self.context.is_some()
            && self
            .context
            .clone()
            .unwrap()
            .starts_with("hub.home.playlists")
    }

    pub fn is_collection_hub(&self) -> bool {
        self.is_hub()
            && self.context.is_some()
            && self
                .context
                .clone()
                .unwrap()
                .starts_with("hub.custom.collection")
    }

    pub fn set_children(&mut self, value: Vec<MetaData>) {
        let len: i32 = value.len().try_into().unwrap();
        if !self.metadata.is_empty() {
            self.metadata = value;
        } else if !self.video.is_empty() {
            self.video = value;
        } else if !self.directory.is_empty() {
            self.directory = value;
        };
        self.size = Some(len);
    }

    pub fn children(&mut self) -> Vec<MetaData> {
        if !self.metadata.is_empty() {
            return self.metadata.clone();
        } else if !self.video.is_empty() {
            return self.video.clone();
        } else if !self.directory.is_empty() {
            return self.directory.clone();
        };
        vec![]
    }

    pub fn children_mut(&mut self) -> &mut Vec<MetaData> {
        if !self.metadata.is_empty() {
            return &mut self.metadata;
        } else if !self.video.is_empty() {
            return &mut self.video;
        } else if !self.directory.is_empty() {
            return &mut self.directory;
        };
        &mut self.metadata
    }
}
