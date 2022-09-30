pub trait Gadget {
    fn render(&mut self, ctx: &egui::Context);
}
