use chrono::Local;
use egui::RichText;
use utils::Gadget;

pub struct ClockGadget;

impl Gadget for ClockGadget {
    fn new(_network_runtime: &utils::NetworkRuntime, _egui_ctx: &egui::Context) -> Self {
        Self
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clock").resizable(false).show(ctx, |ui| {
            let now = Local::now();
            let text = RichText::new(now.format("%H:%M").to_string()).size(64.0);
            ui.label(text);
        });
    }
}
