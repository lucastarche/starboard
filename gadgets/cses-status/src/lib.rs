use std::sync::{Arc, Mutex};

use cses_query::{Problem, ProblemStatus};
use egui::{Color32, CursorIcon, RichText, Sense, TextEdit};
use serde::{Deserialize, Serialize};
use utils::{Gadget, GadgetFactory, MutexExt};

mod cses_query;

pub struct CSESStatusGadget {
    id: usize,
    problem_data: ProblemData,
    network_runtime: utils::NetworkRuntime,
    should_ask_for_id: bool,
    input: String,
}

type ProblemData = Arc<Mutex<Vec<Problem>>>;

pub struct CSESStatusGadgetFactory;

#[derive(Serialize, Deserialize)]
struct CSESStatusConfig {
    #[serde(rename = "user-id")]
    user_id: u64,
}

impl CSESStatusGadget {
    fn ask_for_id(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Configure your CSES UID: "));

            let text_color = self.input.parse::<u64>().err().map(|_| Color32::RED);

            let text_input = TextEdit::singleline(&mut self.input).text_color_opt(text_color);
            let response = ui.add(text_input);

            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if let Ok(user_id) = self.input.parse::<u64>() {
                    utils::update_config_for_gadget(self, CSESStatusConfig { user_id }).unwrap();
                    self.network_runtime.spawn(get_problem_data(
                        self.problem_data.clone(),
                        ui.ctx().clone(),
                        user_id,
                    ));
                    self.should_ask_for_id = false;
                } else {
                    // Do nothing this frame, next one the text will become red
                }
            }
        });
    }
}

impl Gadget for CSESStatusGadget {
    fn id(&self) -> &'static str {
        "cses-status"
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("CSES Status")
            .id(self.make_id(self.id))
            .min_width(200.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if self.should_ask_for_id {
                        self.ask_for_id(ui);
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            render_problems(ui, &self.problem_data.locked())
                        });
                    }
                });
            });
    }
}

impl GadgetFactory for CSESStatusGadgetFactory {
    fn gadget_name(&self) -> &'static str {
        "CSES Status"
    }

    fn make_gadget(
        &self,
        network_runtime: &utils::NetworkRuntime,
        egui_ctx: &egui::Context,
        id: usize,
    ) -> Box<dyn Gadget> {
        let mut cses_status_gadget = CSESStatusGadget {
            id,
            problem_data: Arc::new(Mutex::new(vec![])),
            network_runtime: network_runtime.clone(),
            should_ask_for_id: false,
            input: String::new(),
        };

        let config = utils::config_for_gadget::<CSESStatusConfig>(&cses_status_gadget);

        if let Ok(config) = config {
            network_runtime.spawn(get_problem_data(
                cses_status_gadget.problem_data.clone(),
                egui_ctx.clone(),
                config.user_id,
            ));
        } else {
            cses_status_gadget.should_ask_for_id = true;
        }

        Box::new(cses_status_gadget)
    }
}

async fn get_problem_data(problem_lock: ProblemData, ctx: egui::Context, user_id: u64) {
    let problem_data = cses_query::query_user_status(user_id).await;

    match problem_data {
        Ok(problem_data) => *problem_lock.locked() = problem_data,
        Err(e) => println!("Failed to get user status from cses.fi: {e}"),
    }

    ctx.request_repaint();
}

fn render_problems(ui: &mut egui::Ui, problems: &[Problem]) {
    for problem in problems.iter() {
        if problem.task_link.is_empty() {
            let text = RichText::new("\u{FF01}").color(Color32::RED).size(24.0);
            ui.add_sized([32.0, 32.0], egui::Label::new(text));
        } else {
            let (status_text, background_color) = match problem.status {
                ProblemStatus::Pending => ("\u{2013}", Color32::GRAY), // Dash
                ProblemStatus::Attempted => ("\u{274c}", Color32::LIGHT_RED), // Cross
                ProblemStatus::Completed => ("\u{2705}", Color32::LIGHT_GREEN), // Check
            };
            let text = RichText::new(status_text)
                .color(background_color)
                .size(24.0);

            let item = ui
                .add_sized([32.0, 32.0], egui::Label::new(text).sense(Sense::click()))
                .on_hover_cursor(CursorIcon::PointingHand)
                .on_hover_text(&problem.title);

            if item.clicked() {
                let problem_url = format!("https://cses.fi{}", problem.task_link);
                ui.ctx().output().open_url(problem_url);
            }
        }
    }
}
