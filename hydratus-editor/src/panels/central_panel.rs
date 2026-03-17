use crate::asset_manager::FileUri;

pub fn central_panel(ctx: &egui::Context, background: &mut Option<FileUri>, selected_tile: &mut Option<String>) {
    egui::CentralPanel::default().show(
        ctx, |ui| {
            if let Some(background) = background {
                ui.image(&background.uri);
                
            }
        }
    );
}