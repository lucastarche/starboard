pub trait Drawable {
    fn draw(&mut self, ui: &mut egui::Ui);
}
