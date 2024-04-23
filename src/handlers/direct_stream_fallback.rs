use http::StatusCode;
use salvo::prelude::*;

use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::utils::{add_query_param_salvo, url_from_request};

/// Fallback for direct play failures. It tries to switch to direct stream in case of specific errors or conditions.
#[handler]
pub async fn handler(
    req: &mut Request,
    _res: &mut Response,
) -> Result<(), anyhow::Error> {
    // Extract configuration and request parameters.
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);

    // Check if "directPlay" is explicitly set to "1" (true).
    if req.queries().get("directPlay").unwrap_or(&"1".to_string()) != "1" {
        return Ok(());
    }

    // Build the URL for the upstream request.
    let url = url_from_request(req).to_string();

    // Perform the upstream request.
    let upstream_res = plex_client.get(&url).await?;

    match upstream_res.status() {
        StatusCode::OK => {
            let container =
                MediaContainer::from_reqwest_response(upstream_res).await?;

            // Check for specific decision codes that indicate a need for fallback.
            if let Some(2000) = container.general_decision_code {
                tracing::debug!(
                    "Direct play not available, falling back to direct stream."
                );
                set_direct_stream(req);
            }
        }
        StatusCode::BAD_REQUEST => {
            tracing::debug!(
                "Got 400 bad request, falling back to direct stream."
            );
            set_direct_stream(req);
        }
        status => {
            tracing::error!(status = ?status, "Failed to get plex response");
            return Err(
                salvo::http::StatusError::internal_server_error().into()
            );
        }
    };

    Ok(())
}

/// Helper function to set query parameters for direct stream fallback.
fn set_direct_stream(req: &mut Request) {
    add_query_param_salvo(req, "directPlay".to_string(), "0".to_string());
    add_query_param_salvo(req, "directStream".to_string(), "1".to_string());
}
