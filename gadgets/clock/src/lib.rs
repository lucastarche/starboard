use chrono::Local;
use egui::RichText;
use utils::{Gadget, GadgetFactory};

pub struct ClockGadget;

pub struct ClockGadgetFactory;

impl Gadget for ClockGadget {
    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clock").resizable(false).show(ctx, |ui| {
            let now = Local::now();
            let text = RichText::new(now.format("%H:%M").to_string()).size(64.0);
            ui.label(text);
        });
    }
}

impl GadgetFactory for ClockGadgetFactory {
    fn gadget_name(&self) -> &'static str {
        "Clock"
    }

    fn make_gadget(
        &self,
        _network_runtime: &utils::NetworkRuntime,
        _egui_ctx: &egui::Context,
    ) -> Box<dyn Gadget> {
        Box::new(ClockGadget)
    }
}
