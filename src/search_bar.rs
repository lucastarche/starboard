use egui::Color32;

use crate::gadgets;

#[derive(Default)]
pub struct SearchBar {
    is_open: bool,
    search_query: String,
}

impl SearchBar {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        egui::Window::new("search-bar")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| self.draw_search_bar(ui));
            });
    }

    fn draw_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.search_query);
        let possible_gadgets = gadgets::GADGET_FACTORIES.iter().filter_map(|gadget| {
            if gadget.gadget_name().starts_with(&self.search_query) {
                Some(gadget.gadget_name())
            } else {
                None
            }
        });

        let mut any_matched = false;
        for gadget in possible_gadgets {
            any_matched = true;
            ui.label(gadget);
        }

        if !any_matched {
            ui.colored_label(Color32::RED, "Could not find any gadget");
        }
    }
}
