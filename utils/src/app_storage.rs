use std::path::PathBuf;

pub(crate) fn get_config_path() -> PathBuf {
    let path = dirs::config_dir().unwrap().join("starboard");

    // We ignore errors here for now
    let _ = std::fs::create_dir_all(&path);

    path.join("config.toml")
}

pub(crate) fn get_user_relevant_data_path() -> PathBuf {
    let path = dirs::document_dir().unwrap().join("starboard");

    // We ignore errors here for now
    let _ = std::fs::create_dir_all(&path);

    path
}
