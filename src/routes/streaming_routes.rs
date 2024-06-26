use crate::config::Config;

use salvo::http::header::CONTENT_TYPE;
use salvo::prelude::*;

pub fn routes() -> Router {
    let config = Config::load();

    if !config.redirect_streams.enabled {
        // If redirect_streams is false, return an empty router
        return Router::new();
    }

    Router::new()
        .push(
            Router::with_path(
                "/video/<colon:colon>/transcode/universal/session/<**rest>",
            )
            .get(redirect_stream),
        )
        .push(
            Router::with_path(
                "/library/parts/<itemid>/<partid>/file.<extension>",
            )
            .get(redirect_stream),
        )
}

#[handler]
async fn redirect_stream(req: &mut Request, res: &mut Response) {
    let config = Config::load();
    let redirect_url = config
        .redirect_streams
        .host
        .clone()
        .or_else(|| Some(config.host.clone()));

    let path_and_query = req
        .uri()
        .path_and_query()
        .expect("Request must have a path and query")
        .as_str(); // Safely extract the string representation

    let redirect_url = format!("{:?}{}", redirect_url, path_and_query);

    let mime = mime_guess::from_path(req.uri().path()).first_or_octet_stream();
    res.headers_mut()
        .insert(CONTENT_TYPE, mime.as_ref().parse().unwrap());
    res.render(Redirect::temporary(redirect_url));
}
