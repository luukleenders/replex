use replex_common::{enum_derives, enum_imports};
use replex_deps::*;

enum_imports!();

#[enum_derives]
pub enum ContentType {
    #[strum(serialize = "application/json", serialize = "text/json")]
    Json,

    #[default]
    #[strum(serialize = "text/xml;charset=utf-8", serialize = "application/xml")]
    Xml,
}

#[enum_derives]
pub enum Platform {
    Android,
    Safari,
    Chrome,
    Roku,
    Web,

    #[serde(rename = "iOS")]
    #[strum(serialize = "iOS")]
    Ios,

    #[serde(rename = "tvOS")]
    #[strum(serialize = "tvOS")]
    TvOS,

    #[default]
    #[serde(other)]
    #[strum(serialize = "Generic")]
    Generic,
}

#[enum_derives]
pub enum Style {
    #[serde(rename = "hero")]
    Hero,

    #[default]
    #[serde(rename = "shelf")]
    Shelf,
}

#[enum_derives]
pub enum DeviceType {
    Mobile,

    #[default]
    Tv,
}

impl DeviceType {
    pub fn from_product(product: String) -> DeviceType {
        match product.to_lowercase() {
            x if x.contains("(tv)") => DeviceType::Tv,
            _ => DeviceType::Mobile,
        }
    }
}
