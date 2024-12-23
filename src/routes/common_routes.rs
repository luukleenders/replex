use std::time::Duration;

use salvo::prelude::*;

use crate::config::Config;
use crate::handlers::*;
use crate::middlewares::Timeout;

pub const HERO_IMAGE: &str = "/replex/image/hero/<type>/<uuid>/<token>";
pub const HUBS_CONTINUE_WATCHING: &str = "/hubs/continueWatching";
pub const HUBS_PROMOTED: &str = "/hubs/promoted";
pub const HUBS_SECTIONS: &str = "/hubs/sections/<id>";
pub const REPLEX_COLLECTION_CHILDREN: &str = "/replex/<style>/library/collections/<ids>/children";
pub const REPLEX_DEFAULT: &str = "/replex/<style>/<**rest>";
pub const LIBRARY_METADATA_RELATED: &str = "/library/metadata/<id>/related";
pub const PHOTO_TRANSCODE: &str = "/photo/<colon:colon>/transcode";
pub const PING: &str = "/ping";
pub const REST: &str = "<**rest>";

pub fn routes() -> Router {
    let config = Config::load();
    Router::new()
        .then(|mut router| {
            if config.better_on_deck.enabled {
                router = router.push(
                    Router::with_path(HUBS_CONTINUE_WATCHING).get(empty_media_container_handler),
                );
            }
            router
        })
        .push(
            Router::new()
                .path(LIBRARY_METADATA_RELATED)
                .hoop(Timeout::new(Duration::from_secs(5)))
                .goal(proxy_request_handler),
        )
        .push(Router::with_path(HERO_IMAGE).get(hero_image_handler))
        .push(Router::with_path(HUBS_PROMOTED).get(promoted_hubs_handler))
        .push(Router::with_path(HUBS_SECTIONS).get(section_hubs_handler))
        .push(Router::with_path(REPLEX_COLLECTION_CHILDREN).get(collection_children_handler))
        .push(Router::with_path(REPLEX_DEFAULT).get(default_handler))
        .push(
            Router::with_path(PING)
                .hoop(force_maximum_quality_handler)
                .get(ping),
        )
        .push(
            Router::with_path(PHOTO_TRANSCODE)
                .hoop(photo_request_handler)
                .hoop(local_media_handler)
                .goal(proxy_request_handler),
        )
        .push(Router::with_path(REST).goal(proxy_request_handler))
}
