use bevy::prelude::*;
use crate::dock_state::DockStateResource;
use bevy_egui::{EguiStartupSet}; // <- correct schedule token

mod viewer;
mod systems;
mod dock_state;
mod widgets;
pub mod theme;
mod resize;
mod tabs;
pub mod icons;

pub struct MonolithUIPlugin;

impl Plugin for MonolithUIPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>());

        app.add_message::<mn_core::AppWindowCommand>()
            .init_resource::<theme::ThemeResource>()
           .init_resource::<DockStateResource>()
           .init_resource::<mn_core::DockData>()
           .init_resource::<mn_core::icons::IconTextures>()
           .add_systems(Startup, (
                theme::configure_theme_startup,
                icons::setup_icon_textures
           ).after(EguiStartupSet::InitContexts))
           .add_systems(bevy_egui::EguiPrimaryContextPass, systems::ui_system);
    }
}