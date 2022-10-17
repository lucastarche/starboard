#[derive(Default)]
pub struct SearchBar {
    is_open: bool,
}

impl SearchBar {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        if self.is_open {
            egui::Window::new("search-bar")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.heading("test");
                });
        }
    }
}
