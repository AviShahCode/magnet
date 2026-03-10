use anyhow::Result;
use serde::Deserialize;
use std::fs;

use super::global::BASE_DIR;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: Option<u16>,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

// maybe make LazyLock
pub fn get_config() -> Result<Config> {
    let path = BASE_DIR.join("static").join("config.toml");
    let config_str = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
