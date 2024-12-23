use std::collections::VecDeque;

use anyhow::Result;
use futures::future::join_all;
use mime::Mime;
use multimap::MultiMap;
// use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};
use salvo::http::HeaderValue;
use url::Url;
use yaserde::ser::to_string as to_xml_str;

use salvo::http::HeaderMap;
use salvo::http::ResBody;
use salvo::Request as SalvoRequest;
pub type HyperResponse = hyper::Response<ResBody>;

use crate::config::Config;
use crate::models::{ContentType, DisplayField, DisplayImage, MediaContainer, Meta, MetaData, MetaType, Platform, SpecialBool};
use crate::plex::client::PlexClient;
use crate::plex::traits::MetaDataChildren;

// struct Retry401;
// impl RetryableStrategy for Retry401 {
//     fn handle(
//         &self,
//         res: &std::result::Result<reqwest::Response, reqwest_middleware::Error>,
//     ) -> Option<Retryable> {
//         match res {
//             Ok(success) if success.status() == 401 => Some(Retryable::Transient),
//             Ok(_success) => None,
//             // otherwise do not retry a successful request
//             Err(error) => default_on_request_failure(error),
//         }
//     }
// }

async fn get_last_viewed_at(plex_client: &PlexClient, initial_rating_key: &str) -> Option<i64> {
    let mut queue = VecDeque::from(vec![initial_rating_key.to_string()]);

    while let Some(rating_key) = queue.pop_front() {
        let mut children = match MetaDataChildren::get(plex_client, &rating_key).await {
            Ok(data) => data,
            Err(_) => continue,
        };

        // Iterate backwards over the children
        for child in children.children().iter().rev() {
            if let Some(last_viewed_at) = child.last_viewed_at {
                // Early exit since we found a recent watched date
                return Some(last_viewed_at);
            }

            // Prepare to recurse only if no last_viewed_at has been found
            if child.viewed_leaf_count.unwrap_or(0) > 0 {
                if let Some(rating_key) = &child.rating_key {
                    queue.push_back(rating_key.clone());
                }
            }
        }
    }

    None
}

pub async fn sort_by_last_viewed(plex_client: &PlexClient, items: &mut [MetaData]) {
    let futures: Vec<_> = items
        .iter_mut()
        .filter(|item| item.last_viewed_at.is_none() && item.rating_key.is_some())
        .map(|item| {
            let rating_key = item.rating_key.as_ref().unwrap().clone();
            async move {
                // Execute the async function to fetch last viewed at date
                let last_viewed_at = get_last_viewed_at(plex_client, &rating_key).await;
                (item, last_viewed_at)
            }
        })
        .collect();

    // Wait for all futures to complete
    let results = join_all(futures).await;

    // Update items with the results
    for (item, last_viewed_at) in results {
        if let Some(last_viewed_at) = last_viewed_at {
            item.last_viewed_at = Some(last_viewed_at);
        }
    }

    items.sort_by(|a, b| {
        let a_last_viewed = a.last_viewed_at;
        let b_last_viewed = b.last_viewed_at;

        if a_last_viewed.is_none() {
            return std::cmp::Ordering::Greater;
        }

        if b_last_viewed.is_none() {
            return std::cmp::Ordering::Less;
        }

        b_last_viewed.unwrap().cmp(&a_last_viewed.unwrap())
    });
}

pub fn get_collection_id_from_child_path(path: String) -> i32 {
    let mut path = path.replace("/library/collections/", "");
    path = path.replace("/children", "");
    path.parse().unwrap()
}

pub fn get_collection_id_from_hub(hub: &MetaData) -> i64 {
    hub.hub_identifier
        .clone()
        .unwrap()
        .split('.')
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn url_from_request(req: &SalvoRequest) -> Url {
    let config = Config::load();
    let path_and_query = req
        .uri()
        .path_and_query()
        .expect("Path and query required")
        .as_str();

    let full_url = format!(
        "{}/{}",
        config.host.clone(),
        path_and_query.trim_start_matches('/')
    );

    Url::parse(&full_url).expect("Failed to parse full URL")
}

pub fn replace_query(query: MultiMap<String, String>, req: &mut SalvoRequest) {
    let mut url = url_from_request(req);

    // Replace existing query parameters with new ones
    url.query_pairs_mut()
        .clear()
        .extend_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())));

    // Update the request's URI with the new URL
    if let Ok(new_uri) = hyper::Uri::try_from(url.as_str()) {
        req.set_uri(new_uri);
    } else {
        tracing::error!("Failed to construct a valid URI from modified URL");
    }
}

pub fn add_query_param_salvo(req: &mut SalvoRequest, param: String, value: String) {
    let mut url = url_from_request(req);

    // Modify the query as needed
    url.query_pairs_mut().append_pair(&param, &value);

    // Update the request's URI with the new query
    if let Ok(new_uri) = hyper::Uri::try_from(url.as_str()) {
        req.set_uri(new_uri);
    } else {
        tracing::error!("Failed to construct a valid URI from modified URL");
    }
}

pub fn get_content_type_from_headers(headers: &HeaderMap<HeaderValue>) -> ContentType {
    // Define a static header value for fallback
    static DEFAULT_HEADER_VALUE: HeaderValue = HeaderValue::from_static("text/xml;charset=utf-8");

    // Try to extract either 'content-type' or 'accept' headers
    let content_type_str = headers
        .get("content-type")
        .or_else(|| headers.get("accept"))
        .unwrap_or(&DEFAULT_HEADER_VALUE) // Directly reference the static value
        .to_str()
        .unwrap_or("text/xml;charset=utf-8"); // Default to XML if header can't be converted to a string

    // Determine content type based on header string
    if content_type_str.contains("application/json") {
        ContentType::Json
    } else {
        ContentType::Xml
    }
}

pub fn mime_to_content_type(mime: Mime) -> ContentType {
    match (mime.type_(), mime.subtype()) {
        (mime::JSON, _) => ContentType::Json,
        (mime::XML, _) => ContentType::Xml,
        _ => ContentType::Xml,
    }
}

pub fn hero_meta(platform: Platform) -> Meta {
    println!("Platform: {:?}", platform);
    let r#type = match platform {
        Platform::Chrome => "hero",
        Platform::Safari => "hero",
        _ => "clip",
    };

    Meta {
        // r#type: None,
        r#type: vec![MetaType {
            active: Some(SpecialBool::new(true)),
            r#type: Some(r#type.to_string()),
            title: Some("Videos".to_string()),
        }],
        // style: Some(Style::Hero.to_string().to_lowercase()),
        // display_fields: vec![],
        display_fields: vec![
            // DisplayField {
            //     r#type: Some("movie".to_string()),
            //     fields: vec!["title".to_string(), "originallyAvailableAt".to_string()],
            // },
            // DisplayField {
            //     r#type: Some("show".to_string()),
            //     fields: vec!["title".to_string(), "originallyAvailableAt".to_string()],
            // },
            DisplayField {
                r#type: Some("clip".to_string()),
                fields: vec!["title".to_string(), "parentTitle".to_string(), "originallyAvailableAt".to_string()],
            },
            // DisplayField {
            //     r#type: Some("mixed".to_string()),
            //     fields: vec!["title".to_string(), "originallyAvailableAt".to_string()],
            // },
        ],
        display_images: vec![
            // DisplayImage {
            //     r#type: Some("hero".to_string()),
            //     image_type: Some("coverArt".to_string()),
            // },
            // DisplayImage {
            //     r#type: Some("mixed".to_string()),
            //     image_type: Some("coverArt".to_string()),
            // },
            DisplayImage {
                r#type: Some("clip".to_string()),
                image_type: Some("coverArt".to_string()),
            },
            // DisplayImage {
            //     r#type: Some("movie".to_string()),
            //     image_type: Some("coverArt".to_string()),
            // },
            // DisplayImage {
            //     r#type: Some("show".to_string()),
            //     image_type: Some("coverArt".to_string()),
            // },
        ],
    }
}

// pub fn from_string(
//     string: String, content_type: mime::Mime,
// ) -> Result<MediaContainer> {
//     // dbg!(&string);
//     // dbg!(&content_type.subtype());
//     let result: MediaContainer =
//         match (content_type.type_(), content_type.subtype()) {
//             (_, mime::JSON) => {
//                 let mut c: MediaContainer =
//                     serde_json::from_str(&string).unwrap();
//                 c.content_type = ContentType::Json;
//                 c
//             }
//             _ => MediaContainerWrapper {
//                 // default to xml
//                 // media_container: from_xml_str(&body_string).unwrap(),
//                 media_container: yaserde::de::from_str(&string).unwrap(),
//                 content_type: ContentType::Xml,
//             },
//             // _ => "attachment",
//         };
//     Ok(result)
// }

// pub fn from_bytes(
//     bytes: bytes::Bytes,
//     content_type: ContentType,
// ) -> Result<MediaContainer, Error> {
//     let result: MediaContainer = match content_type {
//         ContentType::Json => {
//             let mut c: MediaContainer =
//                 serde_json::from_reader(&*bytes).expect("Expected proper json");
//             c.content_type = ContentType::Json;
//             c
//         }
//         ContentType::Xml => MediaContainerWrapper {
//             media_container: yaserde::de::from_reader(&*bytes).unwrap(),
//             content_type: ContentType::Xml,
//         },
//     };
//     Ok(result)
// }

// Nice example of extracting response by content type: https://github.com/salvo-rs/salvo/blob/7122c3c009d7b94e7ecf155fb096f11884a8c01b/crates/core/src/test/response.rs#L47
// TODO: use body not string
// pub async fn from_response(
//     mut res: SalvoResponse,
// ) -> Result<MediaContainer> {
//     // let content_type = get_content_type_from_headers(res.headers_mut());
//     let content_type = res.content_type().unwrap();
//     // let bytes = res.take_bytes(res.content_type().as_ref()).await.unwrap();
//     let string = res.take_string().await.unwrap();
//     // dbg!(&res);

//     // let result = match from_bytes(bytes, &content_type) {
//     let result = match from_string(string, content_type) {
//         Ok(result) => result,
//         Err(error) => {
//             error!("Problem deserializing: {:?}", error);
//             let container: MediaContainer = MediaContainerWrapper::default();
//             container // TOOD: Handle this higher up
//         }
//     };
//     Ok(result)
// }

pub async fn to_string(container: MediaContainer, content_type: &ContentType) -> Result<String> {
    match content_type {
        ContentType::Json => Ok(serde_json::to_string(&container).unwrap()),
        // ContentType::Xml => Ok("".to_owned()),
        ContentType::Xml => Ok(to_xml_str(&container).unwrap()),
    }
}

// TODO: Merge hub keys when mixed
pub fn merge_children_keys(mut key_left: String, mut key_right: String) -> String {
    key_left = key_left.replace("/hubs/library/collections/", "");
    key_left = key_left.replace("/library/collections/", "");
    key_left = key_left.replace("/children", "");
    key_right = key_right.replace("/hubs/library/collections/", "");
    key_right = key_right.replace("/library/collections/", "");
    key_right = key_right.replace("/children", "");

    format!(
        "/library/collections/{},{}/children",
        key_left,
        key_right // order is important. As this order is used to generated the library collections
    )
}
