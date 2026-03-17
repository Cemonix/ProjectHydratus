use rfd::FileDialog;

use crate::asset_manager::FileUri;

pub fn menu_bar(ui: &mut egui::Ui, background: &mut Option<FileUri>) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open file").clicked() {
                let file_path = FileDialog::new()
                    .add_filter("image", &["jpeg", "jpg", "png", "bmp"])
                    .set_directory("~")
                    .pick_file();

                if let Some(path) = file_path {
                    *background = match FileUri::try_from(path) {
                        Ok(path) => Some(path),
                        Err(_) => {
                            rfd::MessageDialog::new()
                                .set_level(rfd::MessageLevel::Error)
                                .set_description(String::from("Could not open the file"))
                                .set_title(String::from("Opening file error"))
                                .show();
                            None
                        }
                    };
                }
            }

            if ui.button("Quit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });
}