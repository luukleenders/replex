use crate::models::WrappedMediaContainer;
use crate::plex::models::PlexContext;
use crate::utils::{add_query_param_salvo, get_content_type_from_headers};

use salvo::prelude::*;

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
