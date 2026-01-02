use bevy::prelude::*;
use bevy::math::CompassOctant; // <--- CORRECT IMPORT IS HERE
use crate::dock_state::DockStateResource;

mod viewer;
mod systems;
mod dock_state;
mod widgets;
pub struct MonolithUIPlugin;


#[derive(Message, Clone)]
pub enum AppWindowCommand {
    // Maximize,
    Minimize,
    // Restore,      // un-minimize and/or un-maximize
    ToggleMaximize, // optional toggle semantic
    Shutdown,
    StartMove,
    StartResize(CompassOctant),
}

impl Plugin for MonolithUIPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>());

        app.add_message::<AppWindowCommand>()
           .init_resource::<DockStateResource>()
           .init_resource::<mn_core::DockData>()
           .add_systems(bevy_egui::EguiPrimaryContextPass, systems::ui_system);
    }
}