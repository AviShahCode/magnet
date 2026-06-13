// Global variables that will be used more than once.

use std::{env, path::PathBuf, sync::LazyLock};
use std::fs;

pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let data_dir = PathBuf::from(env::var("MAGNET_DATA_DIR").expect("MAGNET_DATA_DIR environment variable must be set"));
    fs::create_dir_all(data_dir.join("drive")).expect("Failed to create DATA_DIR/drive");
    data_dir
});

pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let data_dir = "/etc/magnet";
    PathBuf::from(data_dir)
});
