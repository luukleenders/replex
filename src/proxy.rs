use std::sync::Arc;
use salvo::{
    async_trait,
    http::{Request, Response},
    proxy::{Proxy as SalvoProxy, ReqwestClient},
    Depot, FlowCtrl, Handler,
};

pub struct Proxy {
    inner: SalvoProxy<StaticUpstream, ReqwestClient>,
}

impl Proxy {
    pub fn new(upstream: Arc<str>, client: reqwest::Client) -> Self {
        let static_upstreams = StaticUpstream::new(upstream);
        let reqwest_client = ReqwestClient::new(client);

        Self {
            inner: SalvoProxy::new(static_upstreams, reqwest_client)
                .url_path_getter(default_url_path_getter)
                .url_query_getter(default_url_query_getter),
        }
    }

    pub async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        self.inner.handle(req, depot, res, ctrl).await;
    }
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

pub struct StaticUpstream(pub Arc<str>);

impl StaticUpstream {
    pub fn new(upstream: Arc<str>) -> Self {
        Self(upstream)
    }
}

#[derive(Debug)]
pub struct StaticUpstreamError {
    pub message: String,
}

impl std::fmt::Display for StaticUpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StaticUpstreamError: {}", self.message)
    }
}

impl std::error::Error for StaticUpstreamError {}

#[async_trait]
impl salvo::proxy::Upstreams for StaticUpstream {
    type Error = StaticUpstreamError;

    fn elect(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<&str, Self::Error>> + Send>> {
        let upstream = self.0.clone(); // Clone the Arc<str> to extend its lifetime.
        Box::pin(async move { Ok(&*upstream) })
    }
}
