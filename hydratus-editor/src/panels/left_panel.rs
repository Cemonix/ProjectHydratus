pub fn left_panel(ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("Left Panel")
        .resizable(true)
        .min_width(160.)
        .max_width(320.)
        .show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();
        });
}