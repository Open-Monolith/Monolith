use bevy::prelude::*;
use crate::dock_state::DockStateResource;

mod viewer;
mod systems;
mod dock_state;
mod widgets;
pub struct MonolithUIPlugin;



impl Plugin for MonolithUIPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>());

        app.init_resource::<DockStateResource>()
           .init_resource::<mn_core::DockData>()
           .add_systems(bevy_egui::EguiPrimaryContextPass, systems::ui_system);
    }
}

