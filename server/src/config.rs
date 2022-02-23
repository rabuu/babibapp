use std::fs;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    general: GeneralConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    ip: String,
    port: u32,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    name: String,
    host: String,
    user: String,
    password: String,
}

impl Config {
    pub fn from_toml(path: &str) -> anyhow::Result<Config> {
        let config_file = fs::read_to_string(path)
            .context(format!("Couldn't load configuration file: `{}`", path))?;
        let config = toml::from_str(&config_file)?;
        Ok(config)
    }
}
