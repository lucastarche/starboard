use egui::{Color32, TextureHandle, Vec2};
use utils::{Drawable, StarboardConfig};

#[derive(Default)]
pub struct AppBackground {
    texture: Option<TextureHandle>,
}

impl Drawable for AppBackground {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            let color_image = StarboardConfig::open().and_then(|config| {
                load_image_from_path(&config.background_path).map_err(anyhow::Error::new)
            });

            if let Ok(color_image) = color_image {
                ui.ctx()
                    .load_texture("background-image", color_image, egui::TextureFilter::Linear)
            } else {
                ui.ctx().load_texture(
                    "background-image",
                    egui::ColorImage::new([1920, 1080], Color32::from_rgb(0x28, 0x28, 0x28)),
                    egui::TextureFilter::Linear,
                )
            }
        });

        ui.image(texture, texture.size_vec2());
    }
}

impl AppBackground {
    pub fn size(&self) -> Vec2 {
        match &self.texture {
            Some(t) => t.size_vec2(),
            None => Vec2::ZERO,
        }
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
