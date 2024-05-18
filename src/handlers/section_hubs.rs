use salvo::prelude::*;

use crate::config::Config;
use crate::models::{MediaContainer, Platform, WrappedMediaContainer};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::transforms::{
    ExcludeWatchedTransform, HideInProgressTransform, HubKeyTransform,
    ReorderHubsTransform, SectionDirectoryTransform, SupplementHubTransform,
    TransformBuilder,
};
use crate::utils::*;

use crate::transforms::HubStyleTransform;

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
) -> Result<(), anyhow::Error> {
    // Extract config and parameters
    let config = Config::load();
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);

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

fn adjust_query_params(req: &mut Request, params: &PlexContext, _config: &Config) {
    let mut count = params.count.unwrap_or(25);
    if params.platform == Platform::Android {
        // Android doesn't do pagination so we to fetch more items.
        count = 50;
    }

    // Always include GUIDs for banners.
    add_query_param_salvo(req, "includeGuids".to_string(), "1".to_string());
    add_query_param_salvo(req, "count".to_string(), count.to_string());
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
    let mut container = MediaContainer::from_reqwest_response(upstream_res).await?;

    TransformBuilder::new(plex_client, params)
        .with_transform(SectionDirectoryTransform)
        .with_transform(HideInProgressTransform)
        .with_transform(ExcludeWatchedTransform)
        .with_transform(SupplementHubTransform)
        .with_transform(ReorderHubsTransform)
        .with_transform(HubStyleTransform { is_home: false })
        .with_transform(HubKeyTransform)
        .apply_to(&mut container)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "Failed to transform media container");
        });

    let result = container.wrap(content_type);

    Ok(result)
}
