//! `config.toml`, read at start-up.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub listen: String,
}

pub fn load(text: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(text)
}
