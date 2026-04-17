use bevy::prelude::*;

use crate::editor::selection::picking::select_with_left_click;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, select_with_left_click);
    }
}