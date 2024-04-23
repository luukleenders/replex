use salvo::prelude::*;

use crate::models::{MediaContainer, WrappedMediaContainer};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::transforms::{
    CollectionStyleTransform, SectionMixTransform, TransformBuilder,
};
use crate::utils::*;

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
) -> Result<(), anyhow::Error> {
    // Extract config and parameters
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);

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

async fn fetch_and_transform_upstream_data(
    req: &Request,
    params: &PlexContext,
    plex_client: &PlexClient,
) -> anyhow::Result<WrappedMediaContainer> {
    let content_type = get_content_type_from_headers(req.headers());
    let collection_ids = req.param::<String>("ids").unwrap();
    let collection_ids: Vec<i64> = collection_ids
        .split(',')
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    // Create a stubbed media container
    let mut container = MediaContainer::default();

    let limit = params.container_size.unwrap_or(50);
    let offset = params.container_start.unwrap_or(0);

    // It's a guessing game
    let is_hub = params.content_directory_id.is_some()
        && !params.include_collections
        && !params.include_advanced
        && !params.exclude_all_leaves;

    TransformBuilder::new(plex_client, params)
        .with_transform(SectionMixTransform {
            collection_ids: collection_ids.clone(),
            offset,
            limit,
        })
        .with_transform(CollectionStyleTransform {
            collection_ids: collection_ids.clone(),
            is_hub,
        })
        // .with_transform(HubKeyTransform)
        .apply_to(&mut container)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "Failed to transform media container");
        });

    let container = container.wrap(content_type);

    Ok(container)
}
