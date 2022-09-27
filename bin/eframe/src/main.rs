fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(StarboardApp::default())),
    );
}

#[derive(Default)]
struct StarboardApp(starboard::MyApp);

impl eframe::App for StarboardApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.0.update(ctx)
    }
}
