use salvo::prelude::*;
use salvo::proxy::{Proxy, ReqwestClient};
use tokio::time::Duration;
use crate::config::Config;

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    let config = Config::load();

    let mut proxy = Proxy::new(
        config.host.clone(),
        ReqwestClient::new(reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 200))
            .build()
            .unwrap())
    );
    proxy = proxy.url_path_getter(default_url_path_getter);
    proxy = proxy.url_query_getter(default_url_query_getter);
    proxy.handle(req, depot, res, ctrl).await;
}

/// Default URL path getter for the proxy.
/// Extracts the path from the incoming request's URI.
fn default_url_path_getter(req: &Request, _depot: &Depot) -> Option<String> {
    Some(req.uri().path().to_string())
}

/// Default URL query getter for the proxy.
/// Extracts the query string from the incoming request's URI, if any.
fn default_url_query_getter(req: &Request, _depot: &Depot) -> Option<String> {
    req.uri().query().map(|q| q.to_string())
}