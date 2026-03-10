use std::path::PathBuf;

use rfd::FileDialog;

pub fn menu_bar(ui: &mut egui::Ui, background: &mut Option<PathBuf>) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open file").clicked() {
                *background = FileDialog::new()
                    .add_filter("image", &["jpeg", "jpg", "png", "bmp"])
                    .set_directory("~")
                    .pick_file();

                
            }

            if ui.button("Quit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });
}