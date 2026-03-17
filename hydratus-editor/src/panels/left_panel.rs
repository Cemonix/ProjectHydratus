pub fn left_panel(ctx: &egui::Context) {
    egui::SidePanel::left("Left Panel")
        .resizable(true)
        .min_width(160.)
        .max_width(320.)
        .show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();
        });
}