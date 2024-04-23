use salvo::prelude::*;

use crate::{
    config::Config,
    handlers::{
        collection_children_handler, default_handler,
        empty_media_container_handler, force_maximum_quality_handler,
        photo_request_handler, ping, promoted_hubs_handler,
        proxy_request_handler, section_hubs_handler,
    },
};

pub fn routes() -> Router {
    let config = Config::load();
    Router::new()
        .then(|mut router| {
            if config.disable_continue_watching || config.better_on_deck {
                router = router.push(
                    Router::with_path("/hubs/continueWatching")
                        .get(empty_media_container_handler),
                );
            }
            router
        })
        .push(
            Router::new()
                .path("/library/metadata/<id>/related")
                // .hoop(Timeout::new(Duration::from_secs(5)))
                .goal(proxy_request_handler),
        )
        .push(Router::with_path("/hubs/promoted").get(promoted_hubs_handler))
        .push(
            Router::with_path("/hubs/sections/<id>").get(section_hubs_handler),
        )
        .push(
            Router::with_path(
                "/replex/<style>/library/collections/<ids>/children",
            )
            .get(collection_children_handler),
        )
        .push(
            Router::with_path("/replex/<style>/<**rest>").get(default_handler),
        )
        .push(
            Router::with_path("/ping")
                .hoop(force_maximum_quality_handler)
                .get(ping),
        )
        .push(
            Router::with_path("/photo/<colon:colon>/transcode")
                .hoop(photo_request_handler)
                .goal(proxy_request_handler),
        )
        .push(Router::with_path("<**rest>").goal(proxy_request_handler))
}
