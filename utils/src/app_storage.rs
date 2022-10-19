use std::path::PathBuf;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StarboardConfig {
    #[serde(rename = "background")]
    pub background_path: Option<PathBuf>,
    #[serde(default = "one")]
    pub background_transparency: f64,
}

impl StarboardConfig {
    pub fn open() -> anyhow::Result<Self> {
        let path = get_config_path();
        let config = std::fs::read_to_string(path)?;

        Ok(toml_edit::easy::from_str(&config)?)
    }
}

/// Avoid calling this, it's just for serde
const fn one() -> f64 {
    1.0
}

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
