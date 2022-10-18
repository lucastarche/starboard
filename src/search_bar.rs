use egui::{Color32, Key, ScrollArea, TextStyle};
use utils::GadgetFactory;

use crate::gadgets;

#[derive(Default)]
pub struct SearchBar {
    is_open: bool,
    is_first_frame: bool,
    search_query: String,
    selected: usize,

    matching_gadgets: Vec<&'static (dyn GadgetFactory + Sync)>,
}

impl SearchBar {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.is_first_frame = true;
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
        ui.style_mut().override_text_style = Some(TextStyle::Heading);
        let response = ui.text_edit_singleline(&mut self.search_query);
        self.handle_input(ui, &response);

        ScrollArea::vertical().show_rows(
            ui,
            ui.text_style_height(&TextStyle::Heading),
            self.matching_gadgets.len(),
            |ui, row_range| {
                for row in row_range {
                    if row == self.selected {
                        ui.colored_label(Color32::YELLOW, self.matching_gadgets[row].gadget_name())
                            .scroll_to_me(None);
                    } else {
                        ui.label(self.matching_gadgets[row].gadget_name());
                    }
                }
            },
        );

        if self.matching_gadgets.is_empty() {
            ui.colored_label(Color32::RED, "Could not find any gadget");
        }
    }

    fn handle_input(&mut self, ui: &egui::Ui, edit_response: &egui::Response) {
        let pressed_tab = ui.input().key_pressed(Key::Tab);
        let pressed_enter = ui.input().key_pressed(Key::Enter);
        let pressed_shift = ui.input().modifiers.shift;
        let should_close = edit_response.lost_focus() && !pressed_tab && !pressed_enter;
        // TODO: Handle enter for opening selected gadget

        if should_close {
            self.is_open = false;
            return;
        }

        if edit_response.changed() || self.is_first_frame {
            self.matching_gadgets = gadgets::GADGET_FACTORIES
                .iter()
                .copied()
                .filter(|gadget| gadget.gadget_name().contains(&self.search_query))
                .collect();

            self.is_first_frame = false;
            self.selected = self.selected.clamp(0, self.matching_gadgets.len() - 1);
        }

        if (pressed_tab && !pressed_shift) || ui.input().key_pressed(Key::ArrowDown) {
            self.select_next();
        }
        if (pressed_tab && pressed_shift) || ui.input().key_pressed(Key::ArrowUp) {
            self.select_prev();
        }

        edit_response.request_focus();
    }

    fn select_next(&mut self) {
        if self.selected + 1 >= self.matching_gadgets.len() {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    fn select_prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.matching_gadgets.len().saturating_sub(1);
        } else {
            self.selected -= 1;
        }
    }
}
