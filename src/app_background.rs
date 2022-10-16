use egui_extras::RetainedImage;
use std::{fs::File, io::Read};
use utils::{
    image::{fit_to_available_size, RetainedImageError},
    Drawable, StarboardConfig,
};

pub struct AppBackground {
    background: Option<RetainedImage>,
}

impl Drawable for AppBackground {
    fn draw(&mut self, ui: &mut egui::Ui) {
        if let Some(background) = &self.background {
            ui.centered_and_justified(|ui| {
                ui.add(fit_to_available_size(ui, background));
            });
        }
    }
}

impl Default for AppBackground {
    fn default() -> Self {
        if let Ok(background) = StarboardConfig::open()
            .and_then(|config| AppBackground::load_background(&config.background_path))
        {
            Self {
                background: Some(background),
            }
        } else {
            Self { background: None }
        }
    }
}

impl AppBackground {
    fn load_background(path: &std::path::Path) -> anyhow::Result<RetainedImage> {
        let f = File::open(path)?;
        let bytes: Vec<u8> = f.bytes().map(|e| e.unwrap_or_default()).collect();
        let image = RetainedImage::from_image_bytes("background-image", bytes.as_slice())
            .map_err(RetainedImageError)?;
        Ok(image)
    }
}
