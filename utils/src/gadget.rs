use crate::NetworkRuntime;

pub trait Gadget {
    fn render(&mut self, ctx: &egui::Context);
}

pub trait GadgetFactory {
    /// The display name of the gadget
    fn gadget_name(&self) -> &'static str;

    /// Create a new gadget
    ///
    /// A network runtime and the egui context are provided for gadgets that need to do more
    /// interesting things.
    fn make_gadget(
        &self,
        network_runtime: &NetworkRuntime,
        egui_ctx: &egui::Context,
    ) -> Box<dyn Gadget>;
}
