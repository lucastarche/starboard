use app_background::AppBackground;
use clock::ClockGadget;
use utils::{Drawable, Gadget};

mod app_background;
#[derive(Default)]
pub struct StarboardApp {
    background: AppBackground,
    clock_gadget: ClockGadget,
}

impl eframe::App for StarboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("background")
            .interactable(false)
            .fixed_pos(ctx.available_rect().center() - self.background.size() / 2.0)
            .order(egui::Order::Background)
            .show(ctx, |ui| self.background.draw(ui));

        self.clock_gadget.render(ctx);
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(StarboardApp::default())),
    );
}
