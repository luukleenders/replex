use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::MediaContainer,
    plex::{client::PlexClient, models::PlexContext, traits::Collection},
    utils::get_content_type_from_headers,
};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    test: String,
}

#[handler]
pub async fn handler(
    req: &mut Request,
    res: &mut Response,
    _depot: &mut Depot,
    _ctrl: &mut FlowCtrl,
) -> Result<(), anyhow::Error> {
    dbg!("test handler");
    let params: PlexContext = req.extract().await?;
    let plex_client = PlexClient::from_request(req, &params);
    let content_type = get_content_type_from_headers(req.headers());

    let mut container = MediaContainer::default();
    let collection = Collection::get(&plex_client, 2108706).await.unwrap();

    // let encoded = bincode::serialize(&collection).unwrap();
    // let decoded: MediaContainer = bincode::deserialize(&encoded).unwrap();

    container.metadata = collection.metadata;
    let result = container.wrap(content_type);
    result.render(res);

    Ok(())
}
