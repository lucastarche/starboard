use egui::TextureHandle;
use std::path::Path;
use utils::Drawable;

#[derive(Default)]
pub struct AppBackground {
    loaded: bool,
    texture: Option<TextureHandle>,
}

impl Drawable for AppBackground {
    fn draw(&mut self, ui: &mut egui::Ui) {
        if !self.loaded {
            if let Ok(color_image) = load_image_from_path(Path::new("assets/background.jpg")) {
                self.texture = Some(ui.ctx().load_texture(
                    "background-image",
                    color_image,
                    egui::TextureFilter::Linear,
                ));
            }

            self.loaded = true;
        }

        if let Some(texture) = &self.texture {
            let available_size = ui.available_size();
            let x_scale = texture.size_vec2().x / available_size.x;
            let y_scale = texture.size_vec2().y / available_size.y;

            ui.centered_and_justified(|ui| {
                ui.image(texture.id(), texture.size_vec2() / x_scale.max(y_scale));
            });
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
