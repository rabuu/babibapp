use std::{fs, net::IpAddr};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpConfig,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub bind: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String,
    pub pool_size: u32,
}

impl Settings {
    pub fn from_toml(path: &str) -> anyhow::Result<Settings> {
        let settings_file = fs::read_to_string(path)
            .context(format!("Couldn't load configuration file: `{}`", path))?;
        let settings = toml::from_str(&settings_file)?;
        Ok(settings)
    }
}
