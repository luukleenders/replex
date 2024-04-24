use itertools::Itertools;
use salvo::prelude::*;

use crate::config::Config;
use crate::models::{MediaContainer, Platform, WrappedMediaContainer};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::transforms::{HubKeyTransform, TransformBuilder};
// use crate::transforms::{SectionDirectoryTransform, ExcludeWatchedTransform};
use crate::utils::*;

use crate::transforms::{
    HubMixTransform, HubStyleTransform, ReorderHubsTransform,
};

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
) -> Result<(), anyhow::Error> {
    let config = Config::load();
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);
    let content_type = get_content_type_from_headers(req.headers());

    // Plex makes a call for every library that is pinned to the home screen.
    // For each library it will request all the hubs that you have set to show on the home screen.
    // The calls include the libraryID but also a list of all the other pinned library IDs.
    //
    // Since we want to mix and reorder the homescreen hubs, we're ignoring every call except the first one.
    // In this call we will fetch all the hubs for every pinned library so we can manipulate them.
    if let (Some(pinned_content_directory_id), Some(content_directory_id)) = (
        &params.pinned_content_directory_id,
        &params.content_directory_id,
    ) {
        if content_directory_id[0] != pinned_content_directory_id[0] {
            res.render(WrappedMediaContainer::empty(content_type));

            return Ok(());
        }
    }

    // Consolidate query parameter modifications
    adjust_query_params(req, &params, config);

    // Perform upstream request and handle response
    match fetch_and_transform_upstream_data(req, &params, &plex_client).await {
        Ok(response) => res.render(response),
        Err(e) => {
            tracing::error!(error = %e, "Failed to process upstream data");
            return Err(e);
        }
    }

    Ok(())
}

fn adjust_query_params(
    req: &mut Request,
    params: &PlexContext,
    _config: &Config,
) {
    if let Some(pinned_id) = &params.pinned_content_directory_id {
        let pinned_ids = pinned_id.iter().join(",");
        add_query_param_salvo(
            req,
            "contentDirectoryID".to_string(),
            pinned_ids,
        );
    }

    // Always include GUIDs for banners.
    add_query_param_salvo(req, "includeGuids".to_string(), "1".to_string());

    // Adjust 'count' based on platform, config, etc.
    let mut count = params.count.unwrap_or(25);
    // let mut count = 10;

    if params.platform == Platform::Android {
        count = 50; // Android-specific adjustment
    }

    // if config.exclude_watched && count < 50 {
    //     count = 50; // General adjustment for excluding watched items
    // }

    dbg!(count);

    add_query_param_salvo(req, "count".to_string(), count.to_string());

    // Add more parameter adjustments as needed.
}

async fn fetch_and_transform_upstream_data(
    req: &Request,
    params: &PlexContext,
    plex_client: &PlexClient,
) -> anyhow::Result<WrappedMediaContainer> {
    let url = url_from_request(req);
    let content_type = get_content_type_from_headers(req.headers());

    // Fetch data from upstream.
    let upstream_res = plex_client.get(url.as_str()).await?;
    let status = upstream_res.status();
    if status != reqwest::StatusCode::OK {
        tracing::error!(status = ?status, "Failed to get plex response");
        return Err(anyhow::anyhow!(
            "Upstream request failed with status: {}",
            status
        ));
    }

    // Deserialize the upstream response.
    let mut container =
        MediaContainer::from_reqwest_response(upstream_res).await?;

    TransformBuilder::new(plex_client, params)
        // .with_transform(SectionDirectoryTransform)
        .with_transform(HubMixTransform)
        .with_transform(ReorderHubsTransform)
        .with_transform(HubStyleTransform { is_home: true })
        // .with_transform(ExcludeWatchedTransform)
        .with_transform(HubKeyTransform)
        .apply_to(&mut container)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "Failed to transform media container");
        });

    let result = container.wrap(content_type);

    Ok(result)
}
