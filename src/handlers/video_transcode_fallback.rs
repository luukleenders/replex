use crate::config::Config;
use crate::models::{MediaContainer, TranscodingStatus};
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::utils::{replace_query, url_from_request};

use salvo::prelude::*;

// TODO: Consider implementing fallback to a version close to the requested bitrate.
// This requires analyzing available versions and their bitrates, comparing them with the requested bitrate,
// and selecting the most appropriate version. The complexity of this feature depends on the availability of detailed
// media version information and might require sophisticated logic to handle edge cases effectively.
#[handler]
pub async fn handler(
    req: &mut Request,
    _res: &mut Response,
) -> Result<(), anyhow::Error> {
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);
    let config = Config::load();
    let original_queries = req.queries().clone();

    let fallback_for = config
        .video_transcode_fallback_for
        .as_deref()
        .unwrap_or_default()[0]
        .to_lowercase();

    let item_key = req
        .queries()
        .get("path")
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    let item = plex_client.clone().get_item_by_key(item_key).await?;

    let media_index = req
        .queries()
        .get("mediaIndex")
        .and_then(|index| index.parse().ok())
        .unwrap_or(0);

    if item.metadata[0].media[media_index]
        .video_resolution
        .as_deref()
        .unwrap_or_default()
        .to_lowercase()
        != fallback_for
    {
        tracing::debug!(
            "Media item not marked for fallback, continuing playback"
        );
        return Ok(());
    }

    if item.metadata[0].media.len() > 1 {
        let status = get_transcoding_for_request(req, &plex_client).await?;

        if status.is_transcoding {
            let fallback_selected =
                execute_fallback_logic(req, &item, media_index, &fallback_for)
                    .await?;
            if !fallback_selected {
                tracing::debug!(
                    "No suitable fallback found, reverting to original query parameters"
                );
                replace_query(original_queries, req);
            }
        }
    } else {
        tracing::debug!("Only one version available, no fallback necessary");
    }

    Ok(())
}

async fn get_transcoding_for_request(
    req: &mut Request,
    plex_client: &PlexClient,
) -> Result<TranscodingStatus, anyhow::Error> {
    let url = url_from_request(req);
    let response = plex_client.get(url.as_str()).await?;
    let transcode = MediaContainer::from_reqwest_response(response).await?;
    let is_transcoding = transcode.metadata.first().map_or(false, |m| {
        m.media.first().map_or(false, |media| {
            media.parts.first().map_or(false, |part| {
                part.streams.iter().any(|s| {
                    s.stream_type == Some(1)
                        && s.decision == Some("transcode".into())
                })
            })
        })
    });

    Ok(TranscodingStatus {
        is_transcoding,
        decision_result: transcode,
    })
}

async fn execute_fallback_logic(
    _req: &mut Request,
    _item: &MediaContainer,
    _media_index: usize,
    _fallback_for: &str,
) -> Result<bool, anyhow::Error> {
    // This function should implement the logic to select and apply a fallback version based on certain criteria.
    // For the purposes of this example, it will simply return Ok(false) to indicate that no fallback was selected.
    // Implement the actual fallback logic here.
    Ok(false)
}
