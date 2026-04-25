use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_egui::EguiPlugin;
pub mod camera;
pub mod editor;
pub mod tools;

use crate::editor::selection::selection_plugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::game()) // ← continuous rendering; no stale frames
        .add_plugins(EguiPlugin::default())
        .add_plugins(new_db::DbPlugin)
        .add_plugins(camera::camera_plugin::AppCameraPlugin)
        .add_plugins(new_ui::UIPlugin)
        .add_plugins(selection_plugin::SelectionPlugin)
        .run();
}