use std::path::{Path, PathBuf};

use hydratus_core::map::Map;
use crate::asset_manager::{AssetManager, FileUri};
use crate::panels::top_panel::top_panel;
use crate::panels::left_panel::left_panel;
use crate::panels::central_panel::central_panel;
use crate::panels::bottom_panel::bottom_panel;

pub struct MapEditor {
    map: Map,
    asset_manager: AssetManager,
    background: Option<FileUri>,
    selected_tile: Option<String>
}

impl MapEditor {
    pub fn new(ctx: &egui::Context) -> Self {
        egui_extras::install_image_loaders(ctx);

        MapEditor {
            map: Map::new(640, 480, 32),
            asset_manager: AssetManager::load_assets(Path::new("hydratus-editor/assets")),
            background: None,
            selected_tile: None
        }
    }
}

impl eframe::App for MapEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top_panel(ctx, &mut self.background);
        left_panel(ctx);
        central_panel(ctx, &mut self.background, &mut self.selected_tile);
        bottom_panel(ctx, &self.asset_manager, &mut self.selected_tile);
    }
}