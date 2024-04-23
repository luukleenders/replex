mod common_routes;
mod streaming_routes;
mod transcoding_routes;

pub use common_routes::routes as common_routes;
pub use streaming_routes::routes as streaming;
pub use transcoding_routes::routes as transcoding;
