use http::header::HeaderName;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PLEX_TOKEN: HeaderName = HeaderName::from_static("x-plex-token");
    pub static ref PLEX_LANGUAGE: HeaderName = HeaderName::from_static("x-plex-language");
    pub static ref PLEX_PLATFORM: HeaderName = HeaderName::from_static("x-plex-platform");
    pub static ref PLEX_CLIENT_IDENTIFIER: HeaderName =
        HeaderName::from_static("x-plex-client-identifier");
    pub static ref PLEX_CLIENT_PROFILE_EXTRA: HeaderName =
        HeaderName::from_static("x-plex-client-profile-extra");
}
