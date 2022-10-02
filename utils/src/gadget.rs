use crate::NetworkRuntime;

pub trait Gadget {
    fn new(network_runtime: &NetworkRuntime, egui_ctx: &egui::Context) -> Self;

    fn render(&mut self, ctx: &egui::Context);
}
