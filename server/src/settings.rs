use std::{fs, net::IpAddr};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub http: HttpSettings,
    pub database: DatabaseSettings,
    pub token: TokenSettings,
    pub root: RootSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpSettings {
    pub bind: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TokenSettings {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RootSettings {
    pub email: String,
    pub password: String,
    pub expiration_minutes: i64,
}

impl Settings {
    pub fn from_toml(path: &str) -> anyhow::Result<Settings> {
        let settings_file = fs::read_to_string(path)
            .context(format!("Couldn't load configuration file: `{}`", path))?;
        let settings = toml::from_str(&settings_file)?;
        Ok(settings)
    }
}
