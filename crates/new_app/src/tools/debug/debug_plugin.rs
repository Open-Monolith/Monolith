use bevy::prelude::*;

use crate::tools::debug::object_place::place_object_here;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            place_object_here
        );
    }
}