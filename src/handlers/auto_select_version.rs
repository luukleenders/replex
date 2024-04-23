use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::utils::replace_query;
use salvo::prelude::*;

#[handler]
pub async fn handler(req: &mut Request) {
    let params: PlexContext = req.extract().await.unwrap_or_default();
    let plex_client = PlexClient::from_request(req, &params);

    if params.screen_resolution.is_empty() {
        tracing::debug!(
            "Skipping auto select as no screen resolution specified"
        );
        return;
    }

    if let Some(media_index) = req.queries().get("mediaIndex") {
        if media_index != "-1" {
            tracing::debug!(
                "Skipping auto select as client specified a media index"
            );
            return;
        }
    }

    if let Some(path) = req.queries().get("path") {
        let item = match plex_client.get_item_by_key(path.to_string()).await {
            Ok(item) => item,
            Err(_) => {
                tracing::debug!("Failed to get item by path: {}", path);
                return;
            }
        };

        let media = &item.metadata[0].media;
        if media.len() <= 1 {
            tracing::debug!(
                "Only one media version available, skipping auto select"
            );
            return;
        }

        let requested_bitrate = req
            .queries()
            .get("videoBitrate")
            .or_else(|| req.queries().get("maxVideoBitrate"))
            .and_then(|v| v.parse::<i64>().ok());

        let device_density = params.screen_resolution[0].height
            * params.screen_resolution[0].width;
        let sorted_media = media
            .iter()
            .filter(|m| m.height.is_some() && m.width.is_some())
            .min_by_key(|m| {
                (device_density - (m.height.unwrap() * m.width.unwrap())).abs()
            });

        if let Some(best_match) = sorted_media {
            let index = media
                .iter()
                .position(|m| m.id == best_match.id)
                .unwrap_or_default();
            tracing::debug!("Auto selected media index: {}", index);

            let mut new_queries = req.queries().clone();
            new_queries.insert("mediaIndex".to_string(), index.to_string());
            if requested_bitrate.is_none() {
                new_queries.insert("directPlay".to_string(), "1".to_string());
            }
            new_queries.insert("subtitles".to_string(), "auto".to_string());
            replace_query(new_queries, req);
        }
    }
}
