use crate::config::Config;
use crate::handlers::{
    auto_select_version_handler, direct_stream_fallback_handler,
    force_maximum_quality_handler, proxy_request_handler,
    video_transcode_fallback_handler,
};
use salvo::prelude::*;

pub fn routes() -> Router {
    let config = Config::load();

    let decision_path = "/video/<colon:colon>/transcode/universal/decision";
    let start_path = "/video/<colon:colon>/transcode/universal/start<**rest>";
    let subtitles_path = "/video/<colon:colon>/transcode/universal/subtitles";

    let mut router = Router::new();

    // Decision router
    let mut decision_router =
        Router::new().path(decision_path).get(proxy_request_handler);
    // Start router
    let mut start_router =
        Router::new().path(start_path).get(proxy_request_handler);
    // Subtitles router
    let mut subtitles_router = Router::new()
        .path(subtitles_path)
        .get(proxy_request_handler);

    // Apply middlewares based on config
    if config.auto_select_version {
        decision_router = decision_router.hoop(auto_select_version_handler);
        start_router = start_router.hoop(auto_select_version_handler);
        subtitles_router = subtitles_router.hoop(auto_select_version_handler);
    }

    if config.force_maximum_quality || config.disable_transcode {
        decision_router = decision_router.hoop(force_maximum_quality_handler);
        start_router = start_router.hoop(force_maximum_quality_handler);
        subtitles_router = subtitles_router.hoop(force_maximum_quality_handler);
    }

    if config.video_transcode_fallback_for.is_some() {
        decision_router =
            decision_router.hoop(video_transcode_fallback_handler);
    }

    decision_router = decision_router.hoop(direct_stream_fallback_handler);

    // Combine all routers
    router = router
        .push(decision_router)
        .push(start_router)
        .push(subtitles_router);

    router
}
