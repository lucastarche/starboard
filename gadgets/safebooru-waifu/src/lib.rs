use std::sync::{Arc, Mutex};

use egui::{CursorIcon, Image, Sense};
use safebooru::{query_random_image, ImageWithMetadata};
use utils::{Gadget, GadgetFactory, MutexExt};

mod safebooru;

pub struct WaifuGadget {
    image: Arc<Mutex<Option<ImageWithMetadata>>>,
}

pub struct WaifuGadgetFactory;

impl Gadget for WaifuGadget {
    fn id(&self) -> &'static str {
        "safebooru-waifu"
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Your daily waifu").show(ctx, |ui| {
            if let Some(image_with_metadata) = &*self.image.locked() {
                render_waifu(ui, image_with_metadata);
            }
        });
    }
}

impl GadgetFactory for WaifuGadgetFactory {
    fn gadget_name(&self) -> &'static str {
        "Safebooru Waifu"
    }

    fn make_gadget(
        &self,
        network_runtime: &utils::NetworkRuntime,
        egui_ctx: &egui::Context,
    ) -> Box<dyn Gadget> {
        let waifu_gadget = WaifuGadget {
            image: Arc::default(),
        };

        let image_lock = waifu_gadget.image.clone();
        let ctx = egui_ctx.clone();
        network_runtime.spawn(async move {
            match query_random_image().await {
                Ok(image) => *image_lock.locked() = Some(image),
                Err(e) => {
                    println!("Failed getting image from safebooru: {e}");
                }
            }

            ctx.request_repaint();
        });

        Box::new(waifu_gadget)
    }
}

fn render_waifu(ui: &mut egui::Ui, image_with_metadata: &ImageWithMetadata) {
    let available_size = ui.available_size();

    let image = &image_with_metadata.inner;
    let x_scale = image.size_vec2().x / available_size.x;
    let y_scale = image.size_vec2().y / available_size.y;

    ui.centered_and_justified(|ui| {
        let button = Image::new(
            image.texture_id(ui.ctx()),
            image.size_vec2() / x_scale.max(y_scale),
        )
        .sense(Sense::click());

        let response = ui.add(button).on_hover_cursor(CursorIcon::PointingHand);

        if response.clicked() {
            ui.ctx()
                .output()
                .open_url(image_with_metadata.post_url.clone());
        }

        response.context_menu(|ui| {
            if ui.button("Copy image URL").clicked() {
                ui.ctx().output().copied_text = image_with_metadata.file_url.clone();
                ui.close_menu();
            }
        });
    });
}
