use egui::{Color32};

use crate::asset_manager::AssetManager;


pub fn bottom_panel(ctx: &egui::Context, asset_manager: &AssetManager, selected_tile: &mut Option<String>) {
    egui::TopBottomPanel::bottom("Bottom panel")
        .min_height(80.0)
        .frame(egui::Frame::default().fill(Color32::from_rgb(95, 95, 95)))
        .show(
        ctx, |ui| {
            ui.add_space(8.);
            ui.horizontal(|ui| {
                ui.add_space(8.);
                for (tile_name, tile_path) in asset_manager.tiles.iter() {
                    if ui.add(
                        egui::Button::image(
                            egui::Image::new(&tile_path.uri)
                                .fit_to_exact_size(egui::Vec2::splat(64.0))
                                .max_size(egui::Vec2::splat(64.0))
                        )
                    ).clicked() {
                        *selected_tile = Some(tile_name.clone());
                    };
                }
            });
        }
    );
}