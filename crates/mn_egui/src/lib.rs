use bevy::prelude::*;
use crate::dock_state::DockStateResource;

mod viewer;
mod systems;
mod dock_state;
mod widgets;
mod theme;
mod resize;
mod tabs;

pub struct MonolithUIPlugin;

impl Plugin for MonolithUIPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>());

        app.add_message::<mn_core::AppWindowCommand>()
           .init_resource::<DockStateResource>()
           .init_resource::<mn_core::DockData>()
           .add_systems(bevy_egui::EguiPrimaryContextPass, systems::ui_system);
    }
}