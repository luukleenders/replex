use crate::models::WrappedMediaContainer;
use crate::plex::models::PlexContext;
use crate::utils::{add_query_param_salvo, get_content_type_from_headers};

use salvo::prelude::*;
use crate::config::Config;
use crate::plex::client::PlexClient;
use crate::plex::traits::HeroArt;

#[handler]
pub async fn ping(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render("pong!")
}

#[handler]
pub async fn empty_media_container_handler(
    req: &mut Request,
    res: &mut Response,
) -> Result<(), anyhow::Error> {
    let content_type = get_content_type_from_headers(req.headers_mut());

    res.render(WrappedMediaContainer::empty(content_type));
    Ok(())
}

// Google tv requests some weird thumbnail for hero elements. Let fix that
#[handler]
pub async fn photo_request_handler(
    req: &mut Request,
    _depot: &mut Depot,
    _res: &mut Response,
) -> Result<(), anyhow::Error> {
    let params: PlexContext = req.extract().await.unwrap();
    if params.size.is_some() && params.clone().size.unwrap().contains('-')
    // (catched things like (medlium-240, large-500),i dont think size paramater orks at all, but who knows
    // && params.platform.is_some()
    // && params.clone().platform.unwrap().to_lowercase() == "android"
    {
        let size: String = params
            .clone()
            .size
            .unwrap()
            .split('-')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        add_query_param_salvo(req, "height".to_string(), size.clone());
        add_query_param_salvo(req, "width".to_string(), size.clone());
        add_query_param_salvo(req, "quality".to_string(), "80".to_string());
    }

    Ok(())
}

#[handler]
pub async fn hero_image_handler(
    req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) -> Result<(), anyhow::Error> {
    // Extract config and parameters
    let _config = Config::load();
    let mut params: PlexContext = req.extract().await?;

    // let _image_type = req.param("type").unwrap();
    let uuid = req.param("uuid").unwrap();
    let token = req.param::<String>("token");

    if token.is_some() {
        params.token = token;
    }

    let plex_client = PlexClient::from_request(req, &params);
    let image_url = HeroArt::get(&plex_client, uuid).await.unwrap_or(None);

    // Check if the image URL is available
    if image_url.is_none() {
        res.status_code(StatusCode::NOT_FOUND);
        return Ok(());
    }

    res.render(Redirect::found(image_url.unwrap()));
    Ok(())
}

#[handler]
pub async fn local_media_handler(
    req: &mut Request,
    _depot: &mut Depot,
    _res: &mut Response,
) {
    let mut params: PlexContext = req.extract().await.unwrap();
    let url = req.query::<String>("url");

    if url.is_some() && url.clone().unwrap().contains("/replex/image/hero") {
        let uri: url::Url = url::Url::parse(url.unwrap().as_str()).unwrap();
        let segments = uri.path_segments().unwrap().collect::<Vec<&str>>();

        let uuid = segments[segments.len() - 2];
        if params.token.is_none() {
            params.token = Some(segments.last().unwrap().to_string());
        }

        let image_url = HeroArt::get(&PlexClient::from_request(req, &params), uuid).await;
        match image_url {
            Ok(image_url) => {
                if image_url.is_none() {
                    return
                }
                add_query_param_salvo(req, "url".to_string(), image_url.unwrap());
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to fetch hero image");
            }
        }
    }
}