use anyhow::{Error as AnyhowError, Result};
use bincode::{Decode, Encode};
use futures_util::Future;
use http::header::{ACCEPT_LANGUAGE, CONNECTION, COOKIE, FORWARDED};
use http::{HeaderMap, HeaderValue, Method};
use reqwest::{header, Url};
use salvo::{Error as SalvoError, Request};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use crate::cache::{CacheManager, CACHE_MANAGER};
use crate::config::Config;
use crate::models::*;

use super::models::PlexContext;

#[derive(Debug, Clone)]
pub struct PlexClient {
    pub http_client: reqwest_middleware::ClientWithMiddleware,
    pub host: String, // TODO: Dont think this supposed to be here. Should be higher up
    pub cache: Arc<CacheManager>,
    // Platform name, e.g. iOS, macOS, etc.
    pub x_plex_platform: Platform,
    // UUID, serial number, or other number unique per device.
    pub x_plex_client_identifier: Option<String>,
    // Auth token for Plex.
    pub x_plex_token: String,
    // Primary name for the device, e.g. "Plex Web (Chrome)".
    // pub x_plex_device_name: String,
}

impl PlexClient {
    pub async fn get(
        &self,
        path_or_url: &str,
    ) -> Result<reqwest::Response, SalvoError> {
        // Check if the input is a valid URL. If parsing fails, it's likely a path.
        let url = match Url::parse(path_or_url) {
            Ok(parsed_url) => parsed_url.to_string(),
            Err(_) => {
                format!(
                    "{}/{}",
                    self.host.trim_end_matches('/'),
                    path_or_url.trim_start_matches('/')
                )
            }
        };

        self.request(Method::GET, &url, None).await
    }

    pub async fn request(
        &self,
        method: http::Method,
        url: &str,
        extra_headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response, SalvoError> {
        let target_uri = url::Url::parse(url).map_err(|e| {
            tracing::error!("Failed to parse URL '{}': {}", url, e);
            AnyhowError::new(e)
        })?;
        // let target_uri = url::Url::parse(url).map_err(AnyhowError::new)?;
        let target_host = target_uri
            .host_str()
            .ok_or_else(|| AnyhowError::msg("Missing host in URL"))?;

        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::HOST,
            header::HeaderValue::from_str(target_host)
                .map_err(AnyhowError::new)?,
        );

        if let Some(extra_headers) = extra_headers {
            headers.extend(extra_headers);
        }

        let response = self
            .http_client
            .request(method, url)
            .headers(headers)
            .send()
            .await
            .map_err(AnyhowError::new)?;

        Ok(response)
    }

    pub async fn cache_or_fetch<T, F, Fut>(
        cache_key: &str,
        fetcher: F,
    ) -> Result<T>
    where
        T: DeserializeOwned + Serialize + 'static + Debug + Decode + Encode,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        // Attempt to retrieve from cache first
        match CACHE_MANAGER.get::<T>(cache_key).await {
            Ok(Some(cached)) => Ok(cached),
            _ => {
                // If not found in cache, invoke the fetcher to get the data
                let result = fetcher().await.map_err(|e| {
                    anyhow::anyhow!("Error fetching data: {}", e)
                })?;

                // Insert fetched data into cache for future requests
                CACHE_MANAGER.insert(cache_key, &result).await.map_err(
                    |e| anyhow::anyhow!("Error inserting into cache: {}", e),
                )?;

                Ok(result)
            }
        }
    }

    pub async fn get_hubs(&self, _id: i32) -> Result<MediaContainer> {
        let resp = self.get("/hubs").await.unwrap();
        let container =
            MediaContainer::from_reqwest_response(resp).await.unwrap();

        Ok(container)
    }

    pub async fn get_item_by_key(self, key: String) -> Result<MediaContainer> {
        let resp = self.get(&key).await.unwrap();
        let container =
            MediaContainer::from_reqwest_response(resp).await.unwrap();

        Ok(container)
    }

    pub async fn get_provider_data(
        &self,
        uuid: &String,
    ) -> Result<MediaContainer> {
        // let config = Config::load();
        let path = format!(
            "https://metadata.provider.plex.tv/library/metadata/{}",
            uuid
        );

        let mut headers = HeaderMap::new();
        // let mut token = config.token.clone();
        headers.insert(
            "X-Plex-Token",
            header::HeaderValue::from_str(self.x_plex_token.clone().as_str())
                .unwrap(),
        );
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );

        let res = self
            .request(Method::GET, &path, Some(headers))
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to get provider data: {}", e)
            })?;

        let container = MediaContainer::from_reqwest_response(res)
            .await
            .map_err(|e| {
                anyhow::anyhow!("Error deserializing response: {}", e)
            })?;

        Ok(container)
    }

    // pub async fn get_provider_data(
    //     &self,
    //     guid: &String,
    // ) -> Result<MediaContainer> {
    //     let path = format!(
    //         "https://metadata.provider.plex.tv/library/metadata/{}",
    //         guid
    //     );
    //
    //     let mut headers = HeaderMap::new();
    //     headers.insert(
    //         "X-Plex-Token",
    //         header::HeaderValue::from_str(self.x_plex_token.clone().as_str())
    //             .unwrap(),
    //     );
    //     headers.insert(
    //         "Accept",
    //         header::HeaderValue::from_static("application/json"),
    //     );
    //
    //     let res = self
    //         .request(Method::GET, &path, Some(headers))
    //         .await
    //         .map_err(|e| {
    //             anyhow::anyhow!("Failed to get provider data: {}", e)
    //         })?;
    //
    //     let container = MediaContainer::from_reqwest_response(res)
    //         .await
    //         .map_err(|e| {
    //             anyhow::anyhow!("Error deserializing response: {}", e)
    //         })?;
    //
    //     Ok(container)
    // }

    pub fn from_request(req: &Request, params: &PlexContext) -> Self {
        let headers = Self::build_headers(params, req.headers());
        let config = Config::load();
        let token = params
            .token
            .as_ref()
            .unwrap_or(&config.token);

        Self {
            http_client: reqwest_middleware::ClientBuilder::new(
                reqwest::Client::builder()
                    .default_headers(headers)
                    .gzip(true)
                    .timeout(Duration::from_secs(30))
                    .build()
                    .expect("Failed to build HTTP client"),
            )
            .build(),
            host: config.host.clone(),
            x_plex_token: token.to_string(),
            x_plex_client_identifier: params.client_identifier.clone(),
            x_plex_platform: params.platform.clone(),
            cache: CACHE_MANAGER.clone(),
        }
    }

    fn header_value(value: &str) -> HeaderValue {
        HeaderValue::from_str(value).expect("Invalid header value")
    }

    pub fn generate_cache_key(&self, name: String) -> String {
        format!(
            "{}:{}-{}",
            name, self.x_plex_token,
            self.x_plex_client_identifier.clone().unwrap_or_default()
        )
    }

    fn build_headers(
        params: &PlexContext,
        req_headers: &HeaderMap,
    ) -> HeaderMap {
        let config = Config::load();
        let mut headers = HeaderMap::new();

        headers.insert("Accept", Self::header_value("application/json"));
        headers.insert(
            "X-Plex-Token",
            Self::header_value(params.token.as_ref().unwrap_or(&config.token)),
        );
        headers.insert(
            "X-Plex-Platform",
            Self::header_value(&params.platform.to_string()),
        );

        if let Some(i) = req_headers.get("X-Plex-Client-Identifier") {
            headers.insert("X-Plex-Client-Identifier", i.clone());
        }

        if let Some(i) = req_headers.get("X-Forwarded-For") {
            headers.insert(FORWARDED, i.clone());
            headers.insert("X-Forwarded-For", i.clone());
            headers.insert("X-Real-IP", i.clone());
        }

        if let Some(i) = req_headers.get("X-Forwarded-Proto") {
            headers.insert("X-Forwarded-Proto", i.clone());
        }

        if let Some(i) = req_headers.get("X-ForwardedHost") {
            headers.insert("X-Forwarded-Host", i.clone());
        }

        if let Some(i) = req_headers.get("X-Forwarded-Port") {
            headers.insert("X-Forwarded-Port", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Session-Id") {
            headers.insert("X-Plex-Session-Id", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Client-Session-Id") {
            headers.insert("X-Plex-Client-Session-Id", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Playback-Id") {
            headers.insert("X-Plex-Playback-Id", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Product") {
            headers.insert("X-Plex-Product", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Version") {
            headers.insert("X-Plex-Version", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Platform-Version") {
            headers.insert("X-Plex-Platform-Version", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Features") {
            headers.insert("X-Plex-Features", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Model") {
            headers.insert("X-Plex-Model", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Device") {
            headers.insert("X-Plex-Device", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Device-Name") {
            headers.insert("X-Plex-Device-Name", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Drm") {
            headers.insert("X-Plex-Drm", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Text-Format") {
            headers.insert("X-Plex-Text-Format", i.clone());
        }

        if let Some(i) = req_headers.get("x-plex-http-pipeline") {
            headers.insert("X-Plex-Http-Pipeline", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Provider-Version") {
            headers.insert("X-Plex-Provider-Version", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Device-Screen-Resolution") {
            headers.insert("X-Plex-Device-Screen-Resolution", i.clone());
        }

        if let Some(i) = req_headers.get("X-Plex-Client-Capabilities") {
            headers.insert("X-Plex-Client-Capabilities", i.clone());
        }

        if let Some(i) = req_headers.get(COOKIE) {
            headers.insert(COOKIE, i.clone());
        }

        if let Some(i) = req_headers.get(ACCEPT_LANGUAGE) {
            headers.insert(ACCEPT_LANGUAGE, i.clone());
        }

        if let Some(i) = req_headers.get(CONNECTION) {
            headers.insert(CONNECTION, i.clone());
        }

        headers
    }
}
