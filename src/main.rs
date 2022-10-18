use app_background::AppBackground;
use search_bar::SearchBar;
use utils::{Drawable, Gadget, NetworkRuntime};

mod app_background;
mod gadgets;
mod search_bar;

pub struct StarboardApp {
    network_runtime: NetworkRuntime,

    background: AppBackground,
    search_bar: SearchBar,
    gadgets: Vec<Box<dyn Gadget>>,
}

impl StarboardApp {
    fn new(_egui_ctx: &egui::Context) -> Self {
        let network_runtime = setup_network_runtime();

        Self {
            network_runtime,
            background: AppBackground::default(),
            search_bar: SearchBar::default(),
            gadgets: vec![],
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
                .push(factory.make_gadget(&self.network_runtime, ctx));
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
    let options = eframe::NativeOptions {
        maximized: true,
        decorated: false,
        ..Default::default()
    };

    eframe::run_native(
        "starboard",
        options,
        Box::new(|cc| Box::new(StarboardApp::new(&cc.egui_ctx))),
    );
}
