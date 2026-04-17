use bevy::prelude::*;
use bevy_egui::{EguiPrimaryContextPass, EguiStartupSet};

use crate::dock::{dock_ui_system, setup_dock };
pub struct UIPlugin;

pub mod dock;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_dock).before(EguiStartupSet::InitContexts))
            .add_systems(EguiPrimaryContextPass, dock_ui_system);
    }
}