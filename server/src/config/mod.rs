// Similar to global variables, but are read from config files.
// TODO: fail early when misconfigured

use crate::config::constants::Constants;
use crate::utils::global::CONFIG_DIR;
use anyhow::{Context, Result};
use std::fs;
use std::sync::LazyLock;
use log::info;

pub mod constants;

#[derive(Clone)]
pub struct Config {
    pub constants: Constants,
}

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| load_configs().expect("Failed to load configs"));

fn load_configs() -> Result<Config> {
    let constants_path = CONFIG_DIR.join("constants.toml");
    let constants_str = fs::read_to_string(&constants_path)
        .with_context(|| format!("Failed to read {:?}", constants_path))?;
    let constants: Constants = toml::from_str(&constants_str)
        .with_context(|| format!("Failed to parse {:?}", constants_path))?;
    info!("Loaded constants config");

    info!("Loaded all configs");
    Ok(Config { constants })
}
