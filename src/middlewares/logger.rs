//! Logging middleware
use std::time::Instant;

use salvo::http::{Request, Response, StatusCode};
use salvo::{async_trait, Depot, FlowCtrl, Handler};
use tracing::{Instrument, Level};

use crate::utils::url_from_request;

/// A simple logger middleware.
pub struct Logger;

#[async_trait]
impl Handler for Logger {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let headers = req.headers_mut().clone();
        let span = tracing::span!(
            Level::DEBUG,
            "Request",
            remote_addr = %req.remote_addr().to_string(),
            version = ?req.version(),
            method = %req.method(),
            headers = ?headers,
            path = %req.uri(),
            span.kind = "server",
            service.name = "replex",
            name = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
            otel.status_description = tracing::field::Empty,
        );

        async move {
            let now = Instant::now();
            let url_in = url_from_request(req);

            ctrl.call_next(req, depot, res).await;
            let duration = now.elapsed();
            let status = res.status_code.unwrap_or(StatusCode::OK);
            let url_out = url_from_request(req);

            // dbg!("START DEBUG CHUNK");
            // dbg!(&url_in);
            // dbg!(&url_out);
            // dbg!("END DEBUG CHUNK");

            tracing::debug!(
                status = %status,
                duration = ?duration,
                url_in = %url_in,
                url_out = %url_out,
                "Response"
            );
        }
        .instrument(span)
        .await
    }
}
