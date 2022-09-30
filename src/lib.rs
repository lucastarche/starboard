use app_background::AppBackground;
use clock::ClockGadget;
use utils::{Drawable, Gadget};

mod app_background;
#[derive(Default)]
pub struct MyApp {
    background: AppBackground,
    clock_gadget: ClockGadget,
}

impl MyApp {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::Area::new("background")
            .interactable(false)
            .fixed_pos(ctx.available_rect().center() - self.background.size() / 2.0)
            .order(egui::Order::Background)
            .show(ctx, |ui| self.background.draw(ui));

        self.clock_gadget.render(ctx);
    }
}
