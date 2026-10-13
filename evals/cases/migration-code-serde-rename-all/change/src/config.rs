//! `config.toml`, read at start-up.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub listen: String,
    /// The currency of orders that do not name one (ISO 4217).
    pub default_currency: String,
}

pub fn load(text: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(text)
}
