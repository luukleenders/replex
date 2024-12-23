mod common_handlers;

mod auto_select_version;
mod collection_children;
mod default;
mod direct_stream_fallback;
mod force_maximum_quality;
mod promoted_hubs;
mod proxy_request;
mod section_hubs;
mod test;
mod video_transcode_fallback;

pub use common_handlers::{
    empty_media_container_handler, hero_image_handler, photo_request_handler, ping, local_media_handler,
};

pub use auto_select_version::handler as auto_select_version_handler;
pub use collection_children::handler as collection_children_handler;
pub use default::handler as default_handler;
pub use direct_stream_fallback::handler as direct_stream_fallback_handler;
pub use force_maximum_quality::handler as force_maximum_quality_handler;
pub use promoted_hubs::handler as promoted_hubs_handler;
pub use proxy_request::handler as proxy_request_handler;
pub use section_hubs::handler as section_hubs_handler;
pub use test::handler as test_handler;
pub use video_transcode_fallback::handler as video_transcode_fallback_handler;
