use crate::config::Config;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::utils::replace_query;

use salvo::prelude::*;

/// Forces the maximum video quality based on various conditions.
#[handler]
pub async fn handler(req: &mut Request) -> Result<(), anyhow::Error> {
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);
    let config = Config::load();
    let mut queries = req.queries().clone();

    // If bitrate limitations are present, clear them and set quality to maximum.
    if queries.get("maxVideoBitrate").is_some()
        || queries.get("videoBitrate").is_some()
    {
        queries.remove("maxVideoBitrate");
        queries.remove("videoBitrate");
    }

    // Adjust query parameters to ensure highest quality and direct play/stream.
    queries.insert("autoAdjustQuality".to_string(), "0".to_string());
    queries.insert("directStream".to_string(), "1".to_string());
    queries.insert("directPlay".to_string(), "1".to_string());
    queries.insert("videoQuality".to_string(), "100".to_string()); // Set quality to 100%

    // Correct media buffer size format if necessary.
    if let Some(size) = queries.get("mediaBufferSize") {
        let corrected_size = size
            .parse::<f32>()
            .ok()
            .map(|size| (size as i64).to_string()) // Convert the f32 to i64 and then to String
            .unwrap_or_else(|| "default_size".to_string()); // Provide a default size or handle None case

        queries.insert("mediaBufferSize".to_string(), corrected_size);
    }

    // Filter out bitrate limitations from the client profile extra query.
    if let Some(extra) = queries.get("X-Plex-Client-Profile-Extra") {
        let filtered_extra = extra
            .split('+')
            .filter(|s| {
                !s.contains("add-limitation")
                    && !s.to_lowercase().contains("name=video.bitrate")
            })
            .collect::<Vec<_>>()
            .join("+");
        queries
            .insert("X-Plex-Client-Profile-Extra".to_string(), filtered_extra);
    }

    // Optionally force direct play for specific video resolutions.
    if let Some(force_resos) = &config.force_direct_play_for {
        if let Some(path) = queries.get("path") {
            let item = plex_client.get_item_by_key(path.to_string()).await?;
            let media_index = queries
                .get("mediaIndex")
                .and_then(|index| index.parse::<usize>().ok())
                .unwrap_or(0);
            let media_item = &item.metadata[0].media[media_index];

            // Use iter().any() to check if the current resolution matches any of the forced resolutions.
            if force_resos.iter().any(|reso| {
                media_item.video_resolution.as_deref() == Some(reso.as_str())
            }) {
                queries.insert("directPlay".to_string(), "1".to_string());
            }
        }
    }

    // Apply the modified queries to the request.
    replace_query(queries, req);
    Ok(())
}
