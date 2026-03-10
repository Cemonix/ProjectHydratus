use std::path::PathBuf;

pub fn central_panel(ctx: &egui::Context, frame: &mut eframe::Frame, background: &mut Option<PathBuf>) {
    egui::CentralPanel::default().show(
        ctx, |ui| {
            if let Some(background) = background {
                ui.image(format!("file://{}", background.display()));
            }
        }
    );
}