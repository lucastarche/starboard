use utils::Gadget;

#[derive(Default)]
pub struct ClockGadget;

impl Gadget for ClockGadget {
    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clock").show(ctx, |ui| {
            ui.label("13:37");
        });
    }
}
