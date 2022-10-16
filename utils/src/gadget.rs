use std::path::PathBuf;

use toml_edit::Item;

use crate::{app_storage, NetworkRuntime};

pub trait Gadget {
    /// An id representing your gadget to be used in config sections or the filesystem
    fn id(&self) -> &'static str;

    fn render(&mut self, ctx: &egui::Context);
}

pub trait GadgetFactory {
    /// The display name of the gadget
    fn gadget_name(&self) -> &'static str;

    /// Create a new gadget
    ///
    /// A network runtime and the egui context are provided for gadgets that need to do more
    /// interesting things.
    fn make_gadget(
        &self,
        network_runtime: &NetworkRuntime,
        egui_ctx: &egui::Context,
    ) -> Box<dyn Gadget>;
}

pub fn config_for_gadget<C>(gadget: &dyn Gadget) -> anyhow::Result<C>
where
    C: serde::de::DeserializeOwned,
{
    let config_path = app_storage::get_config_path();

    let config = std::fs::read_to_string(config_path)?;
    let config = config.parse::<toml_edit::Document>()?;
    match config.get(gadget.id()) {
        Some(config) => Ok(toml_edit::de::from_item(config.clone())?),
        None => anyhow::bail!("Missing section: {} in config file", gadget.id()),
    }
}

pub fn update_config_for_gadget<C>(gadget: &dyn Gadget, c: C) -> anyhow::Result<()>
where
    C: serde::Serialize,
{
    let config_path = app_storage::get_config_path();

    let config = std::fs::read_to_string(&config_path).unwrap_or_default();
    let mut config = config.parse::<toml_edit::Document>()?;
    let c = toml_edit::ser::to_item(&c)?.into_table().unwrap();
    config[gadget.id()] = Item::Table(c);

    std::fs::write(config_path, config.to_string())?;

    Ok(())
}

pub fn user_data_dir_for_gadget(gadget: &dyn Gadget) -> PathBuf {
    let path = app_storage::get_user_relevant_data_path().join(gadget.id());

    // We ignore errors here for now
    let _ = std::fs::create_dir_all(&path);

    path
}
