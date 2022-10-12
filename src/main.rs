use app_background::AppBackground;
use clock::ClockGadget;
use cses_status::CSESStatusGadget;
use utils::{Drawable, Gadget, NetworkRuntime};
use weather::WeatherGadget;

mod app_background;

pub struct StarboardApp {
    background: AppBackground,
    clock_gadget: ClockGadget,
    weather_gadget: WeatherGadget,
    cses_status_gadget: CSESStatusGadget,
}

impl StarboardApp {
    fn new(network_runtime: NetworkRuntime, egui_ctx: egui::Context) -> Self {
        Self {
            background: Default::default(),
            clock_gadget: ClockGadget::new(&network_runtime, &egui_ctx),
            weather_gadget: WeatherGadget::new(&network_runtime, &egui_ctx),
            cses_status_gadget: CSESStatusGadget::new(&network_runtime, &egui_ctx),
        }
    }
}

impl eframe::App for StarboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("background")
            .interactable(false)
            .fixed_pos(ctx.available_rect().center() - self.background.size() / 2.0)
            .order(egui::Order::Background)
            .show(ctx, |ui| self.background.draw(ui));

        self.clock_gadget.render(ctx);
        self.weather_gadget.render(ctx);
        self.cses_status_gadget.render(ctx);
    }
}

fn setup_network_runtime() -> NetworkRuntime {
    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::Builder::new()
        .name("network".into())
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            tx.send(runtime.handle().clone())
                .expect("the other end of this sender shouldn't be gone already");
            runtime
                .block_on(async { tokio::time::sleep(std::time::Duration::from_secs(30)).await });
        })
        .expect("failed to spawn thead");

    rx.blocking_recv().unwrap()
}

fn main() {
    let options = eframe::NativeOptions::default();
    let network_runtime = setup_network_runtime();

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(StarboardApp::new(network_runtime, cc.egui_ctx.clone()))),
    );
}
