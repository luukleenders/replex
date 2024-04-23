use figment::util::bool_from_str_or_int;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{self, Deserialize};

use crate::deserializers::{
    deserialize_comma_separated, deserialize_host,
    vec_from_comma_separated_or_list,
};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_host")]
    pub host: Option<String>,

    pub port: Option<u64>,

    #[serde(default, deserialize_with = "vec_from_comma_separated_or_list")]
    pub exclude_watched: Option<Vec<String>>,

    #[serde(default = "default_cache_ttl")]
    pub cache_ttl: u64,

    #[serde(default = "as_true", deserialize_with = "bool_from_str_or_int")]
    pub cache_rows: bool,

    #[serde(default = "as_true", deserialize_with = "bool_from_str_or_int")]
    pub cache_rows_refresh: bool,

    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    pub hero_rows: Option<Vec<String>>,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub better_on_deck: bool,

    pub in_progress: Option<String>,

    pub next_up: Option<String>,

    #[serde(default, deserialize_with = "vec_from_comma_separated_or_list")]
    pub priority_hubs: Option<Vec<String>>,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub disable_continue_watching: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub disable_user_state: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub disable_leaf_count: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub redirect_streams: bool,

    pub redirect_streams_host: Option<String>,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub disable_related: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub disable_transcode: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub force_maximum_quality: bool,

    #[serde(default, deserialize_with = "bool_from_str_or_int")]
    pub auto_select_version: bool,

    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    pub video_transcode_fallback_for: Option<Vec<String>>,

    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    pub force_direct_play_for: Option<Vec<String>>,

    pub test_script: Option<String>,
}

// #[derive(Debug, Default, PartialEq, Deserialize)]
// pub struct Settings {
//     #[serde(default)]
//     init: Option<String>,
// }

impl Config {
    fn figment() -> Figment {
        Figment::new()
            .merge(Yaml::file("config/config.yml"))
            .merge(Env::prefixed("REPLEX_"))
    }

    pub fn load() -> &'static Self {
        static INSTANCE: Lazy<Config> = Lazy::new(|| {
            let config: Config = Config::figment()
                .extract()
                .expect("Configuration should be correctly set up");

            config
        });
        &INSTANCE
    }
}

fn default_cache_ttl() -> u64 {
    30 * 60
}

fn as_true() -> bool {
    true
}
