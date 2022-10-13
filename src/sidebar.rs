use utils::Drawable;

use crate::gadgets;

pub struct GadgetSidebar;

impl Drawable for GadgetSidebar {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            for gadget_factory in gadgets::GADGET_FACTORIES {
                ui.label(gadget_factory.gadget_name());
            }
        });
    }
}
