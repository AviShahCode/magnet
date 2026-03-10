use std::{env, path::PathBuf, sync::LazyLock};

pub static BASE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let exe = env::current_exe().unwrap();
    exe.parent().unwrap().to_path_buf()
});
