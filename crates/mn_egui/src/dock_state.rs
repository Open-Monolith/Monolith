use bevy::prelude::*;
use egui_dock::{DockState, NodeIndex};
use mn_core::Tab; // Tab comes from mn_core

#[derive(Resource)]
pub struct DockStateResource {
    pub dock_state: DockState<Tab>
}

impl Default for DockStateResource {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![Tab::Viewport]);
        
        let tree = dock_state.main_surface_mut();
        let [viewport, _inspector] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![Tab::Inspector],
        );
        let [_viewport, _hierarchy] = tree.split_left(
            viewport,
            0.2,
            vec![Tab::Hierarchy],
        );

        Self {
            dock_state
        }
    }
}