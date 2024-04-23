use salvo::http::uri::Uri;
use salvo::prelude::*;

pub struct DisableRelatedQuery;

#[async_trait]
impl Handler for DisableRelatedQuery {
    async fn handle(
        &self, req: &mut Request, _depot: &mut Depot, _res: &mut Response, ctrl: &mut FlowCtrl,
    ) {
        let uri = req.uri().clone();
        if let Some(path_and_query) = uri.path_and_query() {
            let path = path_and_query.path();
            let query = path_and_query.query().unwrap_or("");
            let new_query = if query.is_empty() {
                "includeRelated=0".to_string()
            } else {
                format!("{}&includeRelated=0", query)
            };

            // Combine the new query with the original path
            let new_path_and_query = format!("{}?{}", path, new_query);

            // Create a new Uri with the modified path and query
            if let Ok(new_uri) = Uri::try_from(new_path_and_query) {
                *req.uri_mut() = new_uri;
            }
        }

        ctrl.call_next(req, _depot, _res).await;
    }
}
