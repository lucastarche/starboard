use app_background::AppBackground;
use drawable::Drawable;

mod app_background;
mod drawable;

#[derive(Default)]
pub struct MyApp {
    background: AppBackground,
}

impl MyApp {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::Area::new("background")
            .interactable(false)
            .fixed_pos(ctx.available_rect().center() - self.background.size() / 2.0)
            .order(egui::Order::Background)
            .show(ctx, |ui| self.background.draw(ui));
    }
}
