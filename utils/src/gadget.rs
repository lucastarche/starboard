use crate::NetworkRuntime;

pub trait Gadget {
    fn new(network_runtime: &NetworkRuntime) -> Self;

    fn render(&mut self, ctx: &egui::Context);
}
