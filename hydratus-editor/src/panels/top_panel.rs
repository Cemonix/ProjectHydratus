use std::path::PathBuf;

use crate::components::menu::menu_bar;

pub fn top_panel(ctx: &egui::Context, _frame: &mut eframe::Frame, background: &mut Option<PathBuf>) {
    egui::TopBottomPanel::top("Top panel").show(
        ctx, |ui| {
            menu_bar(ui, background);
        }
    );
}