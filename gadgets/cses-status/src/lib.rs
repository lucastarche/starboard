use std::sync::{Arc, Mutex};

use cses_query::{Problem, ProblemStatus};
use egui::{Color32, CursorIcon, RichText, Sense};
use utils::{Gadget, MutexExt};

mod cses_query;

pub struct CSESStatusGadget {
    problem_data: Arc<Mutex<Vec<Problem>>>,
}

impl Gadget for CSESStatusGadget {
    fn new(network_runtime: &utils::NetworkRuntime, egui_ctx: &egui::Context) -> Self {
        let this = Self {
            problem_data: Arc::new(Mutex::new(vec![])),
        };

        let problem_lock = this.problem_data.clone();
        let ctx = egui_ctx.clone();
        network_runtime.spawn(async move {
            // TODO: Third time we ask for this, allow configuring user ID
            let problem_data = cses_query::query_user_status(5418).await;

            match problem_data {
                Ok(problem_data) => *problem_lock.locked() = problem_data,
                Err(e) => println!("Failed to get user status from cses.fi: {e}"),
            }

            ctx.request_repaint();
        });

        this
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::Window::new("CSES Status")
            .min_width(200.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| render_problems(ui, &self.problem_data.locked()));
                });
            });
    }
}

fn render_problems(ui: &mut egui::Ui, problems: &Vec<Problem>) {
    for problem in problems.iter() {
        if problem.task_link == "" {
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
