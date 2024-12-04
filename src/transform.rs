use crate::config::Config;
use crate::models::*;
use crate::plex::client::PlexClient;
use crate::plex::models::*;
use crate::plex::traits::{Collection, CollectionChildren, SectionCollections};
use crate::utils::*;

use async_recursion::async_recursion;
use async_trait::async_trait;
use futures_util::{
    future::{self},
    stream::FuturesOrdered,
    StreamExt,
};
use itertools::Itertools;
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Scope};
use std::sync::Arc;

#[async_trait]
pub trait Transform: Send + Sync + 'static {
    async fn transform_metadata(
        &self,
        _item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) {
    }
    async fn transform_mediacontainer(
        &self,
        item: MediaContainer,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> MediaContainer {
        item
    }
}

#[async_trait]
pub trait Filter: Send + Sync + 'static {
    async fn filter_metadata(
        &self,
        _item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> bool {
        true
    }
    async fn filter_mediacontainer(
        &self,
        _item: MediaContainer,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> bool {
        true
    }
}

// #[derive(TypedBuilder)]
// #[derive(Default)]
#[derive(Clone)]
pub struct TransformBuilder {
    pub plex_client: PlexClient,
    pub options: PlexContext,
    pub mix: bool,
    pub transforms: Vec<Arc<dyn Transform>>,
    pub filters: Vec<Arc<dyn Filter>>,
}

impl TransformBuilder {
    #[inline]
    pub fn new(plex_client: PlexClient, options: PlexContext) -> Self {
        Self {
            transforms: Vec::new(),
            filters: Vec::new(),
            mix: false,
            plex_client,
            options,
        }
    }

    #[inline]
    pub fn with_transform<T: Transform>(mut self, transform: T) -> Self {
        self.transforms.push(Arc::new(transform));
        // self.transforms.insert(0, Arc::new(transform));
        self
    }

    #[inline]
    pub fn with_filter<T: Filter>(mut self, filter: T) -> Self {
        self.filters.push(Arc::new(filter));
        self
    }

    // pub fn merge(mut self, container_left, container_right) -> Self {
    //     self.mix = true;
    //     self
    // }

    #[async_recursion]
    pub async fn apply_to_metadata(
        &self,
        metadata: &mut Vec<MetaData>,
    ) -> Vec<MetaData> {
        let mut filtered_childs: Vec<MetaData> = vec![];
        'outer: for item in metadata {
            for filter in self.filters.clone() {
                // dbg!("filtering");
                if !filter
                    .filter_metadata(
                        item,
                        self.plex_client.clone(),
                        self.options.clone(),
                    )
                    .await
                {
                    continue 'outer;
                }
            }

            if !item.children().is_empty() {
                // dbg!(item.test().len());
                let childs = self.apply_to_metadata(item.children_mut()).await;
                // dbg!(childs.len());
                // dbg!(&item.test());
                item.set_children(childs);
                // item.set_children(self.apply_to_metadata(item.test()).await);
            }

            filtered_childs.push(item.to_owned());
        }

        return filtered_childs;
        // dbg!(&metadata.len());
    }

    // TODO: join async filters
    pub async fn apply_to(self, container: &mut MediaContainer) {
        for transform in self.transforms.clone() {
            let futures =
                container.children_mut().iter_mut().map(|x: &mut MetaData| {
                    transform.transform_metadata(
                        x,
                        self.plex_client.clone(),
                        self.options.clone(),
                    )
                });

            future::join_all(futures).await;

            // dont use join as it needs ti be executed in order
            // let l = std::cell::RefCell::new(&mut container.media_container);
            container = transform
                .transform_mediacontainer(
                    container,
                    self.plex_client.clone(),
                    self.options.clone(),
                )
                .await;
            // dbg!(container.media_container.size);
        }

        // filter behind transform as transform can load in additional data
        let children = container.children_mut();
        let new_children = self.apply_to_metadata(children).await;
        container.set_children(new_children);

        if container.size.is_some() {
            container.size =
                Some(container.children_mut().len().try_into().unwrap());
        }
    }
}

// const T: usize;
#[derive(Default)]
pub struct CollectionHubPermissionFilter;

#[async_trait]
impl Filter for CollectionHubPermissionFilter {
    async fn filter_metadata(
        &self,
        item: &mut MetaData,
        plex_client: PlexClient,
        _options: PlexContext,
    ) -> bool {
        if item.is_hub() && !item.is_collection_hub() {
            return true;
        }
        if !item.is_hub() {
            return true;
        }
        let section_id: i64 = item.library_section_id.unwrap_or_else(|| {
            item.clone()
                .children()
                .first()
                .unwrap()
                .library_section_id
                .expect("Missing Library section id")
        });
        // dbg!(section_id);
        // let mut custom_collections = plex_client.get_section_collections(section_id).await.unwrap();
        let mut custom_collections =
            SectionCollections::get(&plex_client, section_id)
                .await
                .unwrap();
        // dbg!(&custom_collections);
        let custom_collections_ids: Vec<String> = custom_collections
            .children()
            .iter()
            .map(|c| c.rating_key.clone().unwrap())
            .collect();

        custom_collections_ids.contains(
            &item
                .hub_identifier
                .clone()
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .to_owned(),
        )
    }
}

// #[derive(Default, Debug)]
// pub struct HubStyleTransform;

// #[async_trait]
// impl Transform for HubStyleTransform {
//     async fn transform_metadata(
//         &self,
//         item: &mut MetaData,
//         plex_client: PlexClient,
//         options: PlexContext,
//     ) {
//         if item.is_collection_hub() {
//             let mut collection_details = plex_client
//                 .clone()
//                 .get_cached(
//                     plex_client
//                         .get_collection(get_collection_id_from_hub(item)),
//                     format!("collection:{}", item.key.clone()).to_string(),
//                 )
//                 .await;

//             if collection_details.is_ok() && collection_details.unwrap()
//                     .media_container
//                     .children()
//                     .get(0)
//                     .unwrap()
//                     .has_label("REPLEXHERO".to_string()) {
//                 item.style = Some("hero".to_string());
//                 // item.meta = Some(Meta {
//                 //     r#type: None,
//                 //     display_fields: vec![
//                 //         DisplayField {
//                 //             r#type: Some("movie".to_string()),
//                 //             image_type: Some("coverArt".to_string()),
//                 //             fields: vec![],
//                 //         },
//                 //         // DisplayField {
//                 //         //     r#type: Some("show".to_string()),
//                 //         //     fields: vec!["title".to_string(), "year".to_string()],
//                 //         // },
//                 //     ],
//                 // });
//                 // for android, as it doesnt listen to hero style on home..... clip works
//                 if let Some(platform) = &options.platform {
//                     if platform.to_lowercase() == "android" {
//                         item.r#type = "clip".to_string();
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Default, Debug)]
pub struct ReorderHubsTransform;

#[async_trait]
impl Transform for ReorderHubsTransform {
    async fn transform_mediacontainer(
        &self,
        mut item: MediaContainer,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> MediaContainer {
        let config: Config = Config::figment().extract().unwrap();
        let mut hubs_clone = item.children_mut().clone();

        if let Some(priority_titles) = &config.priority_hubs {
            // Iterate over priority titles in reverse to maintain the correct order when reinserting
            for title in priority_titles.iter().rev() {
                if let Some(pos) =
                    hubs_clone.iter().position(|hub| &hub.title == title)
                {
                    let hub = hubs_clone.remove(pos);
                    hubs_clone.insert(0, hub); // Insert the hub at the beginning of the Vec
                }
            }
        }

        *item.children_mut() = hubs_clone;
        item
    }
}

#[derive(Default, Debug, Clone)]
pub struct LibraryMixTransform {
    pub collection_ids: Vec<u32>,
    pub offset: i32,
    pub limit: i32,
    // pub remove_watched: bool,
}

// #[async_recursion]
// pub async fn load_collection_children(
//     id: u32,
//     offset: i32,
//     limit: i32,
//     original_limit: i32,
//     client: PlexClient,
// ) -> anyhow::Result<MediaContainer> {
//     let config: Config = Config::figment().extract().unwrap();
//     let mut c = client
//         .get_collection_children(id, Some(offset), Some(limit))
//         .await?;

//     if !config.include_watched {
//         c.media_container.children_mut().retain(|x| !x.is_watched());

//         let children_lenght = c.media_container.children_mut().len() as i32;
//         // dbg!(children_lenght);
//         // dbg!(limit);
//         // dbg!(c.media_container.total_size.unwrap());
//         // dbg!(offset);
//         // dbg!(limit);
//         // dbg!(children_lenght);
//         // dbg!(c.media_container.total_size);
//         // dbg!("-------");
//         let total_size = c.media_container.total_size.unwrap();
//         if children_lenght < original_limit
//             && total_size > offset + limit && offset < total_size
//         {
//             //dbg!("recursive");
//             // self.clone().load_collection_children();
//             // load more
//             return load_collection_children(
//                 id,
//                 offset,
//                 limit + 10,
//                 original_limit,
//                 client.clone(),
//             ).await;
//         }
//     }
//     Ok(c)
// }

#[async_trait]
impl Transform for LibraryMixTransform {
    async fn transform_mediacontainer(
        &self,
        mut item: MediaContainer,
        plex_client: PlexClient,
        _options: PlexContext,
    ) -> MediaContainer {
        tracing::info!("LibraryMixTransform");

        let mut children: Vec<MetaData> = vec![];
        let mut total_size = 0;

        for id in self.collection_ids.clone() {
            let mut c = CollectionChildren::get(
                &plex_client,
                id as i64,
                Some(self.offset),
                Some(self.limit),
            )
            .await
            .unwrap();

            let collection =
                Collection::get(&plex_client, id as i64).await.unwrap();

            if collection.exclude_watched() {
                c.children_mut().retain(|x| !x.is_watched());
            }

            total_size += c.children().len() as i32;

            if !children.is_empty() {
                children = children
                    .into_iter()
                    .interleave(c.children())
                    .collect::<Vec<MetaData>>();
            } else {
                children.append(&mut c.children());
            }
        }
        item.total_size = Some(total_size);
        // always metadata
        item.metadata = children;
        item
    }
}

#[derive(Default, Debug)]
pub struct HubChildrenLimitTransform {
    pub limit: i32,
}

#[async_trait]
impl Transform for HubChildrenLimitTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) {
        let len = self.limit as usize;
        if item.is_collection_hub() {
            let mut children = item.children();
            children.truncate(len);
            item.set_children(children);
        }
    }
}

#[derive(Default, Debug)]
pub struct HubStyleTransform {
    pub is_home: bool, // use clip instead of hero for android
}

#[async_trait]
impl Transform for HubStyleTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        plex_client: PlexClient,
        options: PlexContext,
    ) {
        // let config: Config = Config::figment().extract().unwrap();
        // let style = item.style.clone().unwrap_or("".to_string()).to_owned();

        if item.is_hub() {
            // TODO: Check why tries to load non existing collectiin? my guess is no access
            let is_hero =
                item.is_hero(plex_client.clone()).await.unwrap_or(false);
            if is_hero {
                let style = ClientHeroStyle::from_context(options.clone());

                item.style = style.style;

                item.r#type = style.r#type;

                if style.include_meta {
                    item.meta = Some(hero_meta());
                }

                let mut futures = FuturesOrdered::new();
                // let now = Instant::now();

                for mut child in item.children() {
                    if style.child_type.clone().is_some() {
                        child.r#type = style.child_type.clone().unwrap();
                    }

                    let client = plex_client.clone();
                    let _options = options.clone();
                    futures.push_back(async move {
                        let mut c = child.clone();
                        let transform =
                            MediaStyleTransform { style: Style::Hero };
                        transform
                            .transform_metadata(&mut c, client, _options)
                            .await;
                        c
                    });
                }
                let children: Vec<MetaData> = futures.collect().await;
                item.set_children(children);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct HubWatchedTransform;

#[async_trait]
impl Transform for HubWatchedTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        plex_client: PlexClient,
        _options: PlexContext,
    ) {
        // let config: Config = Config::figment().extract().unwrap();

        if item.is_hub() {
            let exclude_watched = item
                .exclude_watched(plex_client.clone())
                .await
                .unwrap_or(false);

            if exclude_watched {
                item.children_mut().retain(|x| !x.is_watched());
            }
        }
    }
}

/// Collections can be called from hubs as a refresh. But also standalone.
/// We need to know if if its hub called and if the hub is hero styled for media.
#[derive(Default, Debug)]
pub struct CollectionStyleTransform {
    pub collection_ids: Vec<u32>,
    pub hub: bool, // if collections is meant for hubs
}

#[async_trait]
impl Transform for CollectionStyleTransform {
    async fn transform_mediacontainer(
        &self,
        mut item: MediaContainer,
        plex_client: PlexClient,
        options: PlexContext,
    ) -> MediaContainer {
        let collection_details =
            Collection::get(&plex_client, self.collection_ids[0] as i64).await;

        if collection_details.is_ok()
            && collection_details
                .unwrap()
                .children()
                .first()
                .unwrap()
                .has_label("REPLEXHERO".to_string())
        {
            // let mut futures = FuturesOrdered::new();
            // let now = Instant::now();

            let style = ClientHeroStyle::from_context(options.clone());

            item.meta = Some(hero_meta());

            let mut futures = FuturesOrdered::new();
            for mut child in item.children() {
                if style.child_type.clone().is_some() {
                    child.r#type = style.child_type.clone().unwrap();
                }

                let client = plex_client.clone();
                let _options = options.clone();
                futures.push_back(async move {
                    let mut c = child.clone();
                    let transform = MediaStyleTransform { style: Style::Hero };
                    transform
                        .transform_metadata(&mut c, client, _options)
                        .await;
                    c
                });
            }
            let children: Vec<MetaData> = futures.collect().await;
            item.set_children(children);
        }
        item
    }
}

#[derive(Default)]
pub struct WatchedFilter;

#[async_trait]
impl Filter for WatchedFilter {
    async fn filter_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> bool {
        let config: Config = Config::figment().extract().unwrap();
        if config.exclude_watched {
            return false;
        }

        if !item.is_hub() {
            return !item.is_watched();
        }
        true
    }
}

#[derive(Default, Debug)]
pub struct HubSectionDirectoryTransform;

/// Some sections return a directory instead of video. We dont want that
#[async_trait]
impl Transform for HubSectionDirectoryTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) {
        if item.is_collection_hub() && !item.directory.is_empty() {
            let childs = item.children();
            item.directory = vec![];
            item.video = childs;
        }

        // if item.is_collection_hub() {
        //     let childs = item.children();
        //     item.metadata = vec![];
        //     item.video = childs;
        // }
    }
}

#[derive(Default, Debug)]
pub struct HubKeyTransform;

/// We point to replex so we can do some transform on the children calls
#[async_trait]
impl Transform for HubKeyTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) {
        // if item.is_hub() {
        //     dbg!(&item.key);
        // }
        // let key = item.key.clone().unwrap();
        if item.is_hub()
            && item.key.is_some()
            && !item.key.clone().unwrap().starts_with("/replex")
        {
            // might already been set by the mixings
            // setting an url argument crashes client. So we use the path
            item.key = Some(format!(
                "/replex/{}{}",
                item.style
                    .clone()
                    .unwrap_or(Style::Shelf.to_string().to_lowercase()),
                item.key.clone().unwrap()
            ));
        }
    }
}

#[derive(Default, Debug)]
pub struct UserStateTransform;

#[async_trait]
impl Transform for UserStateTransform {
    async fn transform_metadata(
        &self,
        item: &mut MetaData,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) {
        let config: Config = Config::figment().extract().unwrap();
        if !config.disable_user_state && !config.disable_leaf_count {
            return;
        }
        if item.is_hub() {
            for child in item.children_mut() {
                if config.disable_user_state {
                    child.user_state = Some(SpecialBool::new(false));
                }
                if config.disable_leaf_count {
                    child.leaf_count = None;
                }
            }
        } else {
            if config.disable_user_state {
                item.user_state = Some(SpecialBool::new(false));
            }
            if config.disable_leaf_count {
                item.leaf_count = None;
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct TestTransform;

#[async_trait]
impl Transform for TestTransform {
    async fn transform_mediacontainer(
        &self,
        mut item: MediaContainer,
        _plex_client: PlexClient,
        _options: PlexContext,
    ) -> MediaContainer {
        for i in item.children_mut() {
            for x in i.children_mut() {
                x.guids = vec![];
            }
        }
        item
    }
}

pub struct TestStruct {
    pub value: i32,
    pub valuee: String,
}

pub struct ScriptingMediaContainer {}

impl MediaContainer {
    pub fn get_size(&mut self) -> i64 {
        // rhai::Dynamic::from(self.size.unwrap_or(()))
        // to_dynamic(self.size).unwrap()
        self.size.unwrap_or(0)
    }
    pub fn set_size(&mut self, value: i64) {
        self.size = Some(value);
    }
}

#[derive(Default, Debug)]
pub struct MediaContainerScriptingTransform;

#[async_trait]
impl Transform for MediaContainerScriptingTransform {
    async fn transform_mediacontainer(
        &self,
        item: MediaContainer,
        _plex_client: PlexClient,
        options: PlexContext,
    ) -> MediaContainer {
        let config: Config = Config::figment().extract().unwrap();
        if config.test_script.is_none() {
            return item;
        }

        let media_container: Dynamic = to_dynamic(item).unwrap();
        let context: Dynamic = to_dynamic(options).unwrap();
        let mut engine = Engine::new();

        engine
            .register_type_with_name::<Dynamic>("MediaContainer")
            .register_type_with_name::<Dynamic>("PlexContext");

        let mut scope = Scope::new();
        scope.push("media_container", media_container);
        scope.push("context", context);

        engine
            .run_file_with_scope(&mut scope, config.test_script.unwrap().into())
            .unwrap();

        from_dynamic::<MediaContainer>(
            &scope.get_value::<Dynamic>("media_container").unwrap(),
        )
        .unwrap()
    }
}

// #[async_trait]
// impl Transform for CollectionStyleTransform {
//     async fn transform_mediacontainer(
//         &self,
//         mut item: MediaContainer,
//         plex_client: PlexClient,
//         options: PlexContext,
//     ) -> MediaContainer {
//         let mut collection_details = plex_client
//             .clone()
//             .get_cached(
//                 plex_client.get_collection(self.collection_ids[0] as i32),
//                 format!("collection:{}", self.collection_ids[0].to_string()),
//             )
//             .await;

//         if collection_details.is_ok()
//             && collection_details
//                 .unwrap()
//                 .media_container
//                 .children()
//                 .get(0)
//                 .unwrap()
//                 .has_label("REPLEXHERO".to_string())
//         {
//             let mut futures = FuturesOrdered::new();
//             // let now = Instant::now();

//             let mut style = PlatformHeroStyle::for_client(
//                 options.platform,
//                 options.product.unwrap_or_default(),
//             );

//             item.meta = Some(Meta {
//                 r#type: None,
//                 display_fields: vec![],
//                 display_images: vec![
//                     DisplayImage {
//                         r#type: Some("hero".to_string()),
//                         image_type: Some("coverArt".to_string()),
//                     },
//                     DisplayImage {
//                         r#type: Some("mixed".to_string()),
//                         image_type: Some("coverArt".to_string()),
//                     },
//                     DisplayImage {
//                         r#type: Some("clip".to_string()),
//                         image_type: Some("coverArt".to_string()),
//                     },
//                     DisplayImage {
//                         r#type: Some("movie".to_string()),
//                         image_type: Some("coverArt".to_string()),
//                     },
//                     DisplayImage {
//                         r#type: Some("show".to_string()),
//                         image_type: Some("coverArt".to_string()),
//                     },
//                 ],
//             });

//             for mut child in item.children() {
//                 if style.child_type.clone().is_some() {
//                     child.r#type = style.child_type.clone().unwrap();
//                 }

//                 //child.r#type = "clip".to_string();
//                 // let style = item.style.clone().unwrap();
//                 let client = plex_client.clone();
//                 futures.push_back(async move {
//                     let mut c = child.clone();

//                     let cover_art = child.get_hero_art(client).await;
//                     if cover_art.is_some() {
//                         c.images = vec![
//                             Image {
//                                 r#type: "coverArt".to_string(),
//                                 url: cover_art.clone().unwrap(),
//                                 alt: Some(c.title.clone()),
//                             },
//                             Image {
//                                 r#type: "coverPoster".to_string(),
//                                 url: cover_art.clone().unwrap(),
//                                 alt: Some(c.title.clone()),
//                             },
//                         ];
//                         if style.cover_art_as_art {
//                             c.art = cover_art.clone();
//                         }

//                         c.thumb = cover_art.clone();
//                     }
//                     c
//                 });
//             }

//             let children: Vec<MetaData> = futures.collect().await;
//             item.set_children(children);
//         }
//         item
//     }
// }
