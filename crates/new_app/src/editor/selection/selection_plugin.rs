use bevy::prelude::*;

use crate::editor::selection::{
    highlight::{
        remove_outline_for_deselected,
        setup_outline_material,
        spawn_outline_for_selected
    },
    picking::{SelectionState, select_with_left_click},
};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Startup, setup_outline_material)
            .add_systems(
                Update,
                (
                    select_with_left_click,
                    spawn_outline_for_selected,
                    remove_outline_for_deselected,
                ),
            );
    }
}
