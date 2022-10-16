use egui::Image;
use egui_extras::RetainedImage;

mod retained_image_err;

pub use retained_image_err::RetainedImageError;

pub fn fit_to_available_size(ui: &egui::Ui, image: &RetainedImage) -> Image {
    let image_size = image.size_vec2();
    let x_scale = image_size.x / ui.available_width();
    let y_scale = image_size.y / ui.available_height();
    let scaling_factor = x_scale.max(y_scale);

    Image::new(
        image.texture_id(ui.ctx()),
        image.size_vec2() / scaling_factor,
    )
}
