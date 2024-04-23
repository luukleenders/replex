use regex::Regex;
use salvo::cors::Cors;
use salvo::prelude::*;
use salvo::routing::PathFilter;

use crate::middlewares::{Logger, Timeout};
// DisableRelatedQuery
use crate::routes::{common_routes, streaming, transcoding};

pub fn main_router() -> Router {
    tracing::info!("Setting up main router");
    // Set up the regex for path parameters
    let guid_regex = Regex::new(":").unwrap();
    PathFilter::register_wisp_regex("colon", guid_regex);

    let cors = Cors::permissive().into_handler();
    let compression = Compression::new().enable_gzip(CompressionLevel::Fastest);

    // Create the base router with global middleware
    Router::new()
        .hoop(cors)
        .hoop(compression)
        .hoop(Logger)
        .hoop(Timeout)
        // .hoop(DisableRelatedQuery)
        // Push the routes for the various endpoints
        .push(common_routes())
        .push(streaming())
        .push(transcoding())
}
