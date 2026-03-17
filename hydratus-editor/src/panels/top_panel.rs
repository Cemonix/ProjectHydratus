use crate::asset_manager::FileUri;
use crate::components::menu::menu_bar;

pub fn top_panel(ctx: &egui::Context, background: &mut Option<FileUri>) {
    egui::TopBottomPanel::top("Top panel").show(
        ctx, |ui| {
            menu_bar(ui, background);
        }
    );
}