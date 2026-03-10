use std::path::PathBuf;

use hydratus_core::map::Map;
use crate::panels::top_panel::top_panel;
use crate::panels::left_panel::left_panel;
use crate::panels::central_panel::central_panel;

pub struct MapEditor {
    map: Map,
    background: Option<PathBuf>
}

impl MapEditor {
    pub fn new(ctx: &egui::Context) -> Self {
        egui_extras::install_image_loaders(ctx);

        MapEditor {
            map: Map::new(640, 480, 32),
            background: None
        }
    }
}

impl eframe::App for MapEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        top_panel(ctx, frame, &mut self.background);
        left_panel(ctx, frame);
        central_panel(ctx, frame, &mut self.background);
    }
}