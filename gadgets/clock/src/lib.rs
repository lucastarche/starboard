use chrono::Local;
use egui::RichText;
use utils::{Gadget, GadgetFactory};

pub struct ClockGadget {
    id: usize,
    is_open: bool,
}

pub struct ClockGadgetFactory;

impl Gadget for ClockGadget {
    fn id(&self) -> &'static str {
        "clock"
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clock")
            .resizable(false)
            .id(self.make_id(self.id))
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                let now = Local::now();
                let text = RichText::new(now.format("%H:%M").to_string()).size(64.0);
                ui.label(text);
            });
    }

    fn is_open(&self) -> bool {
        self.is_open
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
        id: usize,
    ) -> Box<dyn Gadget> {
        Box::new(ClockGadget { id, is_open: true })
    }
}
