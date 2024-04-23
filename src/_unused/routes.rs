use crate::config::Config;
use crate::models::*;
use crate::old_cache::*;
use crate::plex_client::*;
use crate::proxy::Proxy;
use crate::transform::*;
use crate::url::*;
use crate::utils::*;
use itertools::Itertools;
use moka::notification::RemovalCause;
use moka::sync::Cache as MokaCacheSync;
use salvo::http::StatusCode;
use salvo::prelude::*;
use std::sync::Arc;
use tokio::time::Duration;
use url::Url;

pub fn old_routes() -> Router {
    let config: Config = Config::figment().extract().unwrap();

    router = router
        .push(
            Router::new()
                .path(PLEX_HUBS_PROMOTED)
                .hoop(auto_refresh_cache())
                .get(transform_hubs_home),
        )
        .push(
            Router::new()
                .path(format!("{}/<id>", PLEX_HUBS_SECTIONS))
                .hoop(auto_refresh_cache())
                .get(get_hubs_sections),
        )
        //.push(
        //    Router::new()
        //        .path(format!("{}/<id>", PLEX_LIBRARY_METADATA))
        //        .get(get_library_item_metadata),
        //)
        .push(
            Router::new()
                .path("/ping")
                .hoop(force_maximum_quality)
                .get(ping),
        )
        .push(
            Router::new()
                .path("/replex/<style>/library/collections/<ids>/children")
                .hoop(default_cache())
                .get(get_collections_children),
        )
        .push(
            Router::new()
                .path("/replex/<style>/<**rest>")
                .hoop(default_cache())
                .get(default_transform),
        )
        //.push(
        //    Router::new()
        //        .path(format!("/playQueues"))
        //       .post(get_play_queues)
        //)
        .push(
            Router::with_path("/photo/<colon:colon>/transcode")
                .hoop(fix_photo_transcode_request)
                .goal(proxy_request),
        )
        .push(Router::with_path("<**rest>").goal(proxy_request));

    router
}

#[handler]
async fn proxy_request(
    req: &mut Request, res: &mut Response, depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let proxy = Proxy::with_client(
        config.host.clone().unwrap(),
        reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 200))
            .build()
            .unwrap(),
    );

    proxy.handle(req, depot, res, ctrl).await;
}

#[handler]
async fn should_skip(
    req: &mut Request, res: &mut Response, depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    // We do this because of a bug in extract. Which taks the body which is needed for proy
    let queries: PlexContextProduct = req.parse_queries().unwrap();
    let headers: PlexContextProduct = req.parse_headers().unwrap();
    let product: Option<String> = if queries.product.is_some() {
        queries.product
    } else if headers.product.is_some() {
        headers.product
    } else {
        None
    };

    if product.is_some() && product.clone().unwrap().to_lowercase() == "plexamp"
    {
        let config: Config = Config::dynamic(req).extract().unwrap();
        let proxy = Proxy::with_client(
            config.host.clone().unwrap(),
            reqwest::Client::builder()
                .timeout(Duration::from_secs(60 * 200))
                .build()
                .unwrap(),
        );

        proxy.handle(req, depot, res, ctrl).await;
        ctrl.skip_rest();
    }
}

// Google tv requests some weird thumbnail for hero elements. Let fix that
#[handler]
async fn fix_photo_transcode_request(
    req: &mut Request, _depot: &mut Depot, res: &mut Response,
) {
    let params: PlexContext = req.extract().await.unwrap();
    if params.size.is_some() && params.clone().size.unwrap().contains('-')
    // (catched things like (medlium-240, large-500),i dont think size paramater orks at all, but who knows
    // && params.platform.is_some()
    // && params.clone().platform.unwrap().to_lowercase() == "android"
    {
        let size: String = params
            .clone()
            .size
            .unwrap()
            .split('-')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        add_query_param_salvo(req, "height".to_string(), size.clone());
        add_query_param_salvo(req, "width".to_string(), size.clone());
        add_query_param_salvo(req, "quality".to_string(), "80".to_string());
    }
}

#[handler]
async fn disable_related_query(
    req: &mut Request, depot: &mut Depot, res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    add_query_param_salvo(req, "includeRelated".to_string(), "0".to_string());
}

#[handler]
pub async fn empty_handler(
    req: &mut Request, res: &mut Response,
) -> Result<(), anyhow::Error> {
    let content_type = get_content_type_from_headers(req.headers_mut());
    let mut container: MediaContainer = MediaContainerWrapper::default();
    container.content_type = content_type.clone();
    // container.media_container.size = Some(0);
    container.media_container.identifier =
        Some("com.plexapp.plugins.library".to_string());
    res.render(container);
    return Ok(());
}

#[handler]
pub async fn transform_hubs_home(
    req: &mut Request, res: &mut Response,
) -> Result<(), anyhow::Error> {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());

    if params.clone().pinned_content_directory_id.is_some()
        && params.clone().content_directory_id.unwrap()[0]
            != params.clone().pinned_content_directory_id.unwrap()[0]
    {
        // We only fill the first one.
        let mut container: MediaContainer = MediaContainerWrapper::default();
        container.content_type = content_type.clone();
        container.media_container.size = Some(0);
        // container.media_container.allow_sync = Some("1".to_string());
        container.media_container.identifier =
            Some("com.plexapp.plugins.library".to_string());
        res.render(container);
        return Ok(());
    }

    if params.clone().pinned_content_directory_id.is_some() {
        // first directory, load everything here because we wanna reemiiiixxx
        add_query_param_salvo(
            req,
            "contentDirectoryID".to_string(),
            params
                .clone()
                .pinned_content_directory_id
                .clone()
                .unwrap()
                .iter()
                .join(",")
                .to_string(),
        );
    }

    // we want guids for banners
    add_query_param_salvo(req, "includeGuids".to_string(), "1".to_string());

    // we want continue watching
    // add_query_param_salvo(req, "excludeContinueWatching".to_string(), "0".to_string());

    let mut count = params.clone().count.unwrap_or(25);

    // some androids have trouble loading more for hero style. So load more at once
    match params.platform {
        Platform::Android => count = 50,
        _ => (),
    }
    // Hack, as the list could be smaller when removing watched items. So we request more.
    if config.exclude_watched && count < 50 {
        count = 50;
    }

    add_query_param_salvo(req, "count".to_string(), count.to_string());

    let upstream_res = plex_client.request(req).await?;
    match upstream_res.status() {
        reqwest::StatusCode::OK => (),
        status => {
            tracing::error!(status = ?status, res = ?upstream_res, "Failed to get plex response");
            return Err(
                salvo::http::StatusError::internal_server_error().into()
            );
        }
    };

    let mut container: MediaContainer =
        from_reqwest_response(upstream_res).await?;
    container.content_type = content_type;

    TransformBuilder::new(plex_client, params.clone())
        .with_transform(HubStyleTransform { is_home: true })
        // .with_transform(HubSectionDirectoryTransform)
        .with_transform(HubWatchedTransform)
        .with_transform(HubMixTransform)
        // .with_transform(HubChildrenLimitTransform {
        //     limit: params.clone().count.unwrap(),
        // })
        .with_transform(UserStateTransform)
        .with_transform(HubKeyTransform)
        .with_transform(ReorderHubsTransform)
        .apply_to(&mut container)
        .await;

    res.render(container);
    Ok(())
}

#[handler]
pub async fn get_hubs_sections(
    req: &mut Request, res: &mut Response,
) -> Result<(), anyhow::Error> {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());

    let mut count = params.clone().count.unwrap_or(25);

    match params.platform {
        Platform::Android => count = 50,
        _ => (),
    }

    // Hack, as the list could be smaller when removing watched items. So we request more.
    if config.exclude_watched && count < 50 {
        count = 50;
    }

    add_query_param_salvo(req, "count".to_string(), count.to_string());

    // we want guids for banners
    add_query_param_salvo(req, "includeGuids".to_string(), "1".to_string());

    let upstream_res = plex_client.request(req).await.unwrap();
    match upstream_res.status() {
        reqwest::StatusCode::OK => (),
        status => {
            tracing::error!(status = ?status, res = ?upstream_res, "Failed to get plex response");
            return Err(
                salvo::http::StatusError::internal_server_error().into()
            );
        }
    };

    let mut container: MediaContainer =
        from_reqwest_response(upstream_res).await?;
    container.content_type = content_type;

    TransformBuilder::new(plex_client, params.clone())
        .with_transform(HubSectionDirectoryTransform)
        .with_transform(HubStyleTransform { is_home: false })
        .with_transform(HubWatchedTransform)
        .with_transform(UserStateTransform)
        .with_transform(HubKeyTransform)
        //.with_transform(MediaContainerScriptingTransform)
        // .with_filter(CollectionHubPermissionFilter)
        // .with_filter(WatchedFilter)
        .apply_to(&mut container)
        .await;
    // dbg!(container.media_container.count);
    res.render(container);
    Ok(())
}

// pub async fn transform_section(
//     req: &mut Request,
//     _depot: &mut Depot,
//     res: &mut Response,
// ) -> Result<(), anyhow::Error> {

// }

#[handler]
pub async fn get_collections_children(
    req: &mut Request, _depot: &mut Depot, res: &mut Response,
) -> Result<(), anyhow::Error> {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let collection_ids = req.param::<String>("ids").unwrap();
    let collection_ids: Vec<u32> = collection_ids
        .split(',')
        .filter(|&v| !v.parse::<u32>().is_err())
        .map(|v| v.parse().unwrap())
        .collect();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());

    // We dont listen to pagination. We have a hard max of 250 per collection
    let mut limit: i32 = 250;
    let mut offset: i32 = 0;

    // in we dont remove watched then we dont need to limit
    if !config.exclude_watched {
        limit = params.container_size.unwrap_or(50);
        offset = params.container_start.unwrap_or(0);
    }

    // create a stub
    let mut container: MediaContainer = MediaContainerWrapper::default();
    container.content_type = content_type;
    let size = container.media_container.children().len();
    container.media_container.size = Some(size.try_into().unwrap());
    container.media_container.offset = Some(offset);

    // filtering of watched happens in the transform
    TransformBuilder::new(plex_client, params.clone())
        .with_transform(LibraryMixTransform {
            collection_ids: collection_ids.clone(),
            offset,
            limit,
        })
        .with_transform(CollectionStyleTransform {
            collection_ids: collection_ids.clone(),
            hub: params.content_directory_id.is_some() // its a guessing game
                && !params.include_collections
                && !params.include_advanced
                && !params.exclude_all_leaves,
        })
        .with_transform(UserStateTransform)
        //.with_transform(MediaContainerScriptingTransform)
        .apply_to(&mut container)
        .await;

    res.render(container); // TODO: FIx XML
    Ok(())
}

#[handler]
pub async fn default_transform(
    req: &mut Request, _depot: &mut Depot, res: &mut Response,
) -> Result<(), anyhow::Error> {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());
    let style = req.param::<Style>("style").unwrap();
    let rest_path = req.param::<String>("**rest").unwrap();

    // We dont listen to pagination. We have a hard max of 250 per collection
    let mut limit: i32 = 250;
    let mut offset: i32 = 0;

    // in we dont remove watched then we dont need to limit
    if !config.exclude_watched {
        limit = params.container_size.unwrap_or(50);
        offset = params.container_start.unwrap_or(0);
    }

    let mut url = Url::parse(req.uri_mut().to_string().as_str()).unwrap();
    url.set_path(&rest_path);
    req.set_uri(hyper::Uri::try_from(url.as_str()).unwrap());

    let upstream_res = plex_client.request(req).await?;
    match upstream_res.status() {
        reqwest::StatusCode::OK => (),
        status => {
            tracing::error!(status = ?status, res = ?upstream_res, "Failed to get plex response");
            return Err(
                salvo::http::StatusError::internal_server_error().into()
            );
        }
    };

    let mut container: MediaContainer =
        from_reqwest_response(upstream_res).await?;
    container.content_type = content_type;
    // container.media_container.meta

    TransformBuilder::new(plex_client, params.clone())
        .with_transform(MediaStyleTransform { style: style })
        .with_transform(UserStateTransform)
        .apply_to(&mut container)
        .await;

    res.render(container);
    Ok(())
}

#[handler]
pub async fn get_library_item_metadata(req: &mut Request, res: &mut Response) {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());

    if config.disable_related {
        add_query_param_salvo(
            req,
            "includeRelated".to_string(),
            "0".to_string(),
        );
    }

    let upstream_res = plex_client.request(req).await.unwrap();
    let mut container: MediaContainer = match from_reqwest_response(
        upstream_res,
    )
    .await
    {
        Ok(r) => r,
        Err(error) => {
            tracing::error!(error = ?error, uri = ?req.uri(), "Failed to get plex response");
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    container.content_type = content_type;

    TransformBuilder::new(plex_client, params.clone())
        .with_transform(MediaContainerScriptingTransform)
        .apply_to(&mut container)
        .await;
    // dbg!(container.media_container.count);
    res.render(container);
}

#[handler]
pub async fn get_play_queues(req: &mut Request, res: &mut Response) {
    let config: Config = Config::dynamic(req).extract().unwrap();
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let content_type = get_content_type_from_headers(req.headers_mut());

    if config.disable_related {
        add_query_param_salvo(
            req,
            "includeRelated".to_string(),
            "0".to_string(),
        );
    }

    let upstream_res = plex_client.request(req).await.unwrap();
    let mut container: MediaContainer = match from_reqwest_response(
        upstream_res,
    )
    .await
    {
        Ok(r) => r,
        Err(error) => {
            tracing::error!(error = ?error, uri = ?req.uri(), "Failed to get plex response");
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    container.content_type = content_type;

    TransformBuilder::new(plex_client, params.clone())
        .with_transform(MediaContainerScriptingTransform)
        .apply_to(&mut container)
        .await;

    res.render(container);
}

pub fn auto_refresh_cache() -> Cache<MemoryStore<String>, RequestIssuer> {
    let config: Config = Config::figment().extract().unwrap();

    if config.cache_ttl == 0 || !config.cache_rows || !config.cache_rows_refresh
    {
        return default_cache();
    }

    // TODO: Maybe stop after a month? we can add a timestamp header to the key when first cached.
    let listener = move |k: Arc<String>,
                         v: CachedEntry,
                         cause: RemovalCause| {
        if cause != RemovalCause::Expired {
            return;
        }

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "http://{}{}",
            v.req_local_addr
                .to_string()
                .replace("socket://", "")
                .as_str(),
            v.req_uri.path_and_query().unwrap()
        );

        let mut req = client.get(url).headers(v.req_headers);
        tracing::trace!(req = ?req, "Refreshing cached route entry");
        // tracing::trace!("Refreshing cached route entry");

        std::thread::spawn(move || {
            match req.send() {
                Ok(res) => {
                    // dbg!(res);
                    tracing::debug!("Succesfully refreshed cached route entry");
                }
                Err(err) => {
                    tracing::error!(err = ?err, "Failed to refresh cached route entry");
                }
            }
        });
    };

    Cache::new(
        MemoryStore::with_moka_cache(
            MokaCacheSync::builder()
                .time_to_live(Duration::from_secs(config.cache_ttl))
                .eviction_listener(listener)
                .build(),
        ),
        RequestIssuer::with_plex_defaults(),
    )
}

pub fn default_cache() -> Cache<MemoryStore<String>, RequestIssuer> {
    let config: Config = Config::figment().extract().unwrap();
    let ttl = if config.cache_rows {
        config.cache_ttl
    } else {
        0
    };

    Cache::new(
        MemoryStore::with_moka_cache(
            MokaCacheSync::builder()
                .time_to_live(Duration::from_secs(ttl))
                .build(),
        ),
        RequestIssuer::with_plex_defaults(),
    )
}

// async fn execute_video_transcode_fallback(
//     req: &mut Request,
//     item: MediaContainer,
//     media_index: usize,
// ) -> Result<(), anyhow::Error> {
//     let params: PlexContext = req.extract().await.unwrap();
//     let plex_client = PlexClient::from_request(req, params.clone());
//     let mut queries = req.queries().clone();
//     let mut original_queries = req.queries().clone();

//     let response = plex_client.request(req).await?;
//     let mut transcode: MediaContainer =
//         from_reqwest_response(response).await?;

//     let streams =
//         &transcode.media_container.metadata[0].media[media_index].parts[0].streams;
//     let selected_media = transcode.media_container.metadata[0].media[media_index].clone();
//     let mut fallback_selected = false;
//     for stream in streams {
//         if stream.stream_type.clone().unwrap() == 1
//             && stream.decision.clone().unwrap_or("unknown".to_string())
//                 == "transcode"
//         {
//             tracing::trace!(
//                 "{} is transcoding, looking for fallback",
//                 selected_media
//             );
//             // for now just select a random fallback
//             for (index, media) in
//                 item.media_container.metadata[0].media.iter().enumerate()
//             {
//                 if transcode.media_container.metadata[0].media[media_index].id != media.id
//                 {
//                     tracing::debug!(
//                         "Video transcode fallback from {} to {}",
//                         selected_media,
//                         media,
//                     );
//                     queries.remove("mediaIndex");
//                     queries.insert("mediaIndex".to_string(), index.to_string());
//                     queries.remove("directPlay");
//                     queries.insert("directPlay".to_string(), "0".to_string());
//                     queries.remove("directStream");
//                     queries.insert("directStream".to_string(), "1".to_string());
//                     fallback_selected = true;
//                     break;
//                 }
//             }
//         }
//     }
//     if !fallback_selected {
//         replace_query(original_queries, req);
//     }
//     Ok(())
// }

pub struct TranscodingStatus {
    pub is_transcoding: bool,
    pub decision_result: MediaContainer,
}

async fn get_transcoding_for_request(
    req: &mut Request,
) -> Result<TranscodingStatus, anyhow::Error> {
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let response = plex_client.request(req).await?;
    let mut transcode: MediaContainer = from_reqwest_response(response).await?;
    let mut is_transcoding = false;

    if transcode.media_container.size.is_some()
        && transcode.media_container.size.unwrap() == 0
    {
        return Ok(TranscodingStatus {
            is_transcoding,
            decision_result: transcode,
        });
    }

    let streams =
        &transcode.media_container.metadata[0].media[0].parts[0].streams;
    // let selected_media = transcode.media_container.metadata[0].media[0].clone();
    for stream in streams {
        if stream.stream_type.clone().unwrap() == 1
            && stream.decision.clone().unwrap_or("unknown".to_string())
                == "transcode"
        {
            is_transcoding = true;
            break;
        }
    }

    Ok(TranscodingStatus {
        is_transcoding,
        decision_result: transcode,
    })
}

// TODO: Fallback to a version close to the requested bitrate
#[handler]
async fn video_transcode_fallback(
    req: &mut salvo::Request, depot: &mut Depot, res: &mut salvo::Response,
    ctrl: &mut FlowCtrl,
) -> Result<(), anyhow::Error> {
    let params: PlexContext = req.extract().await.unwrap();
    let plex_client = PlexClient::from_request(req, params.clone());
    let config: Config = Config::dynamic(req).extract().unwrap();
    let mut queries = req.queries().clone();
    let mut original_queries = req.queries().clone();
    let media_index: usize = if (req.queries().get("mediaIndex").is_none()
        || req.queries().get("mediaIndex").unwrap() == "-1")
    {
        0
    } else {
        req.queries()
            .get("mediaIndex")
            .unwrap()
            .parse::<usize>()
            .unwrap()
    };

    let fallback_for =
        config.video_transcode_fallback_for.unwrap()[0].to_lowercase();

    let item = plex_client
        .clone()
        .get_item_by_key(req.queries().get("path").unwrap().to_string())
        .await
        .unwrap();

    if item.media_container.metadata[0].media[media_index]
        .video_resolution
        .clone()
        .unwrap()
        .to_lowercase()
        != fallback_for
    {
        tracing::debug!("Media item not marked for fallback, continue playing");
        return Ok(());
    }

    if item.media_container.metadata[0].media.len() <= 1 {
        tracing::debug!("Nothing to fallback on, skipping fallback check");
    } else {
        // execute_video_transcode_fallback(req, item, media_index).await?;
        // let response = plex_client.request(req).await?;
        // let mut transcode: MediaContainer =
        //     from_reqwest_response(response).await?;
        // let streams =
        //     &transcode.media_container.metadata[0].media[0].parts[0].streams;
        // let selected_media =
        //     transcode.media_container.metadata[0].media[0].clone();
        let mut requested_bitrate: Option<i64> = None;
        if queries.get("videoBitrate").is_some() {
            requested_bitrate =
                Some(queries.get("videoBitrate").unwrap().parse().unwrap());
        } else if queries.get("maxVideoBitrate").is_some() {
            requested_bitrate =
                Some(queries.get("maxVideoBitrate").unwrap().parse().unwrap());
        }

        let mut fallback_selected = false;
        // this could fail.
        let status: TranscodingStatus =
            get_transcoding_for_request(req).await?;
        let selected_media = item.media_container.metadata[0].media[0].clone();
        let mut available_media_ids: Vec<i64> = item.media_container.metadata
            [0]
        .media
        .iter()
        .map(|x| x.id)
        .collect();
        available_media_ids.retain(|x| *x != selected_media.id);
        // available_media_ids.remove(selected_media.id);
        if status.is_transcoding {
            tracing::debug!(
                "{} transcoding, looking for fallback",
                selected_media
            );

            let mut media_items =
                item.media_container.metadata[0].media.clone();
            media_items.sort_by(|x, y| {
                let current_density = x.height.unwrap() * x.width.unwrap();
                let next_density = y.height.unwrap() * y.width.unwrap();

                if current_density < next_density {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Less;
                }
            });
            // dbg!(&media_items.iter().map(|x| x.video_resolution.clone()));
            // for now just select a random fallback
            for (index, media) in media_items.iter().enumerate() {
                if available_media_ids.contains(&media.id) {
                    if queries.get("maxVideoBitrate").is_some()
                        || queries.get("videoBitrate").is_some()
                    {
                        // tracing::trace!(
                        //     "Video has max bitrate which always forces transcode. Forcing max quality for fallback {}",
                        //     media,
                        // );

                        // if same resolution we can assume it will transcode again. Fallback to another resolution
                        let resolution = media
                            .video_resolution
                            .clone()
                            .unwrap()
                            .to_lowercase();
                        if resolution == fallback_for {
                            continue;
                        }

                        // check if requested falls into a resolution range. Either we remove the max bitrate or allow it
                        //let requested_bitrate: i64 = queries
                        //    .get("videoBitrate")
                        //    .unwrap_or_else(|| queries.get("maxVideoBitrate").unwrap()).parse().unwrap();

                        //if (resolution == "1080" && requested_bitrate >= 8000)
                        //    || (resolution == "720"
                        //        && requested_bitrate >= 2000)
                        //{
                        //    force_maximum_quality
                        //        .handle(req, depot, res, ctrl)
                        //        .await;
                        //    queries = req.queries().clone();
                        //}
                    }

                    // force_maximum_quality
                    tracing::debug!(
                        "Video transcode fallback from {} to {}",
                        selected_media,
                        media,
                    );
                    // let mut media_queries = req.queries().clone();
                    queries.remove("mediaIndex");
                    queries.insert("mediaIndex".to_string(), index.to_string());
                    queries.remove("directStream");
                    queries.insert("directStream".to_string(), "1".to_string());

                    if requested_bitrate.is_none() {
                        queries.remove("directPlay");
                        queries
                            .insert("directPlay".to_string(), "1".to_string());
                    }

                    queries.remove("subtitles");
                    queries.insert("subtitles".to_string(), "auto".to_string());

                    replace_query(queries.clone(), req);
                    // processed_media_indexes.append(selected_media.id);
                    // available_media_ids.remove(selected_media.id);

                    if media.video_resolution.clone().unwrap().to_lowercase()
                        != fallback_for
                    {
                        fallback_selected = true;
                        break;
                    }

                    let status: TranscodingStatus =
                        get_transcoding_for_request(req).await?;
                    available_media_ids.retain(|x| *x != media.id);
                    if status.is_transcoding && available_media_ids.len() != 0 {
                        tracing::debug!(
                            "Fallback is transcoding, getting another fallback",
                        );
                        continue;
                    }
                    // let mut transcode: MediaContainer =
                    //     from_reqwest_response(response).await?;
                    fallback_selected = true;
                    break;
                }
            }
            if !fallback_selected {
                tracing::debug!("No suitable fallback found");
                replace_query(original_queries, req);
            }
        }
    }

    // replace_query(queries, req);
    Ok(())
}

#[handler]
async fn ping(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render("pong!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use rstest::rstest;
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};
    use std::env;

    #[rstest]
    #[case::hubs_sections(
        "/hubs/sections/6",
        "tests/mock/out/hubs_sections_6.json"
    )]
    #[case::hubs_promoted(
        format!("{}?contentDirectoryID=6&pinnedContentDirectoryID=6,7", PLEX_HUBS_PROMOTED), "tests/mock/out/hubs_promoted_6.json")
    ]
    #[tokio::test]
    async fn test_routes(#[case] path: String, #[case] expected_path: String) {
        let mock_server = get_mock_server();
        env::set_var(
            "REPLEX_HOST",
            format!("http://{}", mock_server.address().to_string()),
        );

        let service = Service::new(super::route());

        let content =
            TestClient::get(format!("http://127.0.0.1:5800/{}", &path))
                .add_header("X-Plex-Token", "fakeID", true)
                .add_header("X-Plex-Client-Identifier", "fakeID", true)
                .add_header("Accept", "application/json", true)
                .send((&service))
                .await
                .take_string()
                .await
                .unwrap();
        assert_eq!(content, "Hello world!");
    }

    #[tokio::test]
    async fn test_hello_world() {
        let mock_server = get_mock_server();
        env::set_var(
            "REPLEX_HOST",
            format!("http://{}", mock_server.address().to_string()),
        );

        let service = Service::new(super::route());

        let content =
            TestClient::get(format!("http://127.0.0.1:5800/{}", "hello"))
                .send((&service))
                .await
                .take_string()
                .await
                .unwrap();
        assert_eq!(content, "Hello world!");
    }
}
