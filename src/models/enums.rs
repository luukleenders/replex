use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use strum_macros::Display as EnumDisplay;
use strum_macros::{Display, EnumString};
use yaserde_derive::{YaDeserialize, YaSerialize};
use bincode::{Decode, Encode};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    EnumString,
    EnumDisplay,
    Serialize,
    Deserialize,
    YaSerialize,
    YaDeserialize,
    Encode,
    Decode,
)]
pub enum ContentType {
    #[strum(serialize = "application/json", serialize = "text/json")]
    Json,
    #[strum(
        serialize = "text/xml;charset=utf-8",
        serialize = "application/xml"
    )]
    #[default]
    Xml,
}

#[derive(
    Debug,
    Default,
    Clone,
    Display,
    PartialEq,
    Eq,
    EnumString,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub enum Platform {
    Android,
    #[serde(rename = "iOS")]
    #[strum(serialize = "iOS")]
    Ios,
    #[serde(rename = "tvOS")]
    #[strum(serialize = "tvOS")]
    TvOS,
    Safari,
    Chrome,
    Roku,
    #[serde(other)]
    #[strum(serialize = "Generic")]
    #[default]
    Generic,
}

#[derive(
    Debug, Clone, Display, PartialEq, Eq, EnumString, Serialize, Deserialize, Encode, Decode,
)]
pub enum Style {
    #[serde(rename = "hero")]
    Hero,
    #[serde(rename = "shelf")]
    Shelf,
}

#[derive(Debug)]
pub enum DeviceType {
    Tv,
    Mobile,
}

impl DeviceType {
    pub fn from_product(product: String) -> DeviceType {
        match product.to_lowercase() {
            x if x.contains("(tv)") => DeviceType::Tv,
            _ => DeviceType::Mobile,
        }
    }
}
