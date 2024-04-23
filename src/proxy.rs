use salvo::BoxedError;
use salvo::{
    async_trait,
    http::{Request, Response},
    proxy::{Proxy as SalvoProxy, Upstreams},
    Depot, FlowCtrl, Handler,
};

pub struct Proxy<U> {
    inner: SalvoProxy<U>,
}

impl<U> Proxy<U>
where
    U: Upstreams,
    U::Error: Into<BoxedError>,
{
    pub fn new(upstreams: U) -> Self {
        Self {
            inner: SalvoProxy::new(upstreams)
                .url_path_getter(default_url_path_getter)
                .url_query_getter(default_url_query_getter),
        }
    }

    pub fn with_client(upstreams: U, client: reqwest::Client) -> Self {
        Self {
            inner: SalvoProxy::with_client(upstreams, client)
                .url_path_getter(default_url_path_getter)
                .url_query_getter(default_url_query_getter),
        }
    }
}

impl<U> Clone for Proxy<U>
where
    U: Upstreams + Clone,
    U::Error: Into<BoxedError>,
{
    fn clone(&self) -> Self {
        let upstreams = self.inner.upstreams.clone();
        let client = self.inner.client.clone();

        Self {
            inner: SalvoProxy::with_client(upstreams, client)
                .url_path_getter(default_url_path_getter)
                .url_query_getter(default_url_query_getter),
        }
    }
}

#[async_trait]
impl<U> Handler for Proxy<U>
where
    U: Upstreams,
    U::Error: Into<BoxedError>,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Implement any required request modifications here
        // For example, to add or modify headers:
        // if let Ok(config) = Config::dynamic(req).extract() {
        //     if let Some(host) = config.host {
        //         req.headers_mut().insert(http::header::HOST, host.parse().unwrap());
        //     }
        // }

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
