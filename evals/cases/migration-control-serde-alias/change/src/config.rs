//! `config.toml`, read at start-up.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    /// The address the HTTP server binds to. Files written before 4.2 call
    /// it `listen`, which is still read.
    #[serde(alias = "listen")]
    pub bind_address: String,
}

pub fn load(text: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(text)
}
