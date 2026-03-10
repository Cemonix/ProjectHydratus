mod editor;
mod panels;
mod components;

use crate::editor::MapEditor;

const EDITOR_WIDTH: f32 = 1024.;
const EDITOR_HEIGHT: f32 = 768.;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([EDITOR_WIDTH, EDITOR_HEIGHT])
            .with_close_button(true)
            .with_maximize_button(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Hydratus Map Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(MapEditor::new(&cc.egui_ctx)))),
    )
}
