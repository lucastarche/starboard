use app_background::AppBackground;
use search_bar::SearchBar;
use utils::{Drawable, Gadget, NetworkRuntime, StarboardConfig};

mod app_background;
mod gadgets;
mod search_bar;

pub struct StarboardApp {
    current_id: usize,
    network_runtime: NetworkRuntime,

    background: Option<AppBackground>,
    search_bar: SearchBar,
    gadgets: Vec<Box<dyn Gadget>>,
}

impl StarboardApp {
    fn new(_egui_ctx: &egui::Context, draw_background: bool) -> Self {
        let network_runtime = setup_network_runtime();

        Self {
            current_id: 0,
            network_runtime,
            background: draw_background.then(AppBackground::default),
            search_bar: SearchBar::default(),
            gadgets: vec![],
        }
    }
}

impl eframe::App for StarboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(background) = &mut self.background {
            egui::Area::new("background")
                .interactable(false)
                .fixed_pos(ctx.available_rect().center() - background.size() / 2.0)
                .order(egui::Order::Background)
                .show(ctx, |ui| background.draw(ui));
        }

        for gadget in &mut self.gadgets {
            gadget.render(ctx);
        }

        // modifiers.command returns true if Ctrl is down in Windows / Linux
        // or Command is down in MacOS
        let ctrl_down = ctx.input().modifiers.command;
        let enter_down = ctx.input().key_pressed(egui::Key::Enter);
        if ctrl_down && enter_down {
            self.search_bar.toggle();
        }

        self.search_bar.update(ctx);

        if let Some(factory) = self.search_bar.add_gadget {
            self.gadgets
                .push(factory.make_gadget(&self.network_runtime, ctx, self.current_id));
            self.current_id += 1;
            self.search_bar.add_gadget = None;
        }
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
    let (transparent_background, draw_background) = StarboardConfig::open()
        .map(|config| {
            (
                config.background_transparency < 1.0,
                // We special case 0.0 transparency in order to not paint at all, which is more efficient
                config.background_transparency != 0.0,
            )
        })
        .unwrap_or_default();

    let options = eframe::NativeOptions {
        maximized: true,
        decorated: false,
        transparent: transparent_background,
        ..Default::default()
    };

    eframe::run_native(
        "starboard",
        options,
        Box::new(move |cc| Box::new(StarboardApp::new(&cc.egui_ctx, draw_background))),
    );
}
