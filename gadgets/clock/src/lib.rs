use chrono::Local;
use egui::RichText;
use utils::Gadget;

#[derive(Default)]
pub struct ClockGadget;

impl Gadget for ClockGadget {
    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clock").show(ctx, |ui| {
            let now = Local::now();
            let text = RichText::new(now.format("%H:%M").to_string()).size(64.0);
            ui.label(text);
        });
    }
}
