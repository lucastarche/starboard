use std::sync::{Arc, Mutex};

use egui_extras::RetainedImage;
use safebooru::query_random_image;
use utils::{Gadget, GadgetFactory, MutexExt};

mod safebooru;

pub struct WaifuGadget {
    image: Arc<Mutex<Option<RetainedImage>>>,
}

pub struct WaifuGadgetFactory;

impl Gadget for WaifuGadget {
    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Your daily waifu").show(ctx, |ui| {
            if let Some(image) = &*self.image.locked() {
                image.show_size(ui, ui.available_size());
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
            image: Arc::new(Mutex::new(None)),
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
