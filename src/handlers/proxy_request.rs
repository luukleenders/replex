use salvo::prelude::*;
use tokio::time::{timeout, Duration};

use crate::config::Config;
use crate::proxy::Proxy;

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    let config = Config::load();
    let base_url = config
        .host
        .clone()
        .unwrap_or_else(|| panic!("Host must be specified in the config"));

    let proxy = Proxy::new(base_url);
    let timeout_duration = Duration::from_secs(60 * 200);
    let proxy_result = timeout(timeout_duration, async {
        proxy.handle(req, depot, res, ctrl).await
    })
    .await;

    // Handle the request with the proxy
    // This will forward the request to the target URL and return the response to the client
    match proxy_result {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Proxy error: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
