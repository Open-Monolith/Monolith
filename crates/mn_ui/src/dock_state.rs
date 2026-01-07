use bevy::prelude::*;
use egui_dock::{DockState, NodeIndex};
use mn_core::{TabKind, MonoTab}; // Tab comes from mn_core

#[derive(Resource)]
pub struct DockStateResource {
    pub dock_state: DockState<MonoTab>,
    pub tab_id_counter: u32,
}

impl  DockStateResource {
    pub fn create_tab(&mut self, kind: TabKind) -> MonoTab {
        self.tab_id_counter += 1;
        MonoTab { 
            kind: kind.clone(),
            title: format!("{:?}", kind),
            id: self.tab_id_counter
        }
    }
}

impl Default for DockStateResource {
    fn default() -> Self {
        let mut local_id_counter: u32 = 1;

        let mut next_tab = |kind: TabKind| -> MonoTab {
            let tab = MonoTab {
                kind: kind.clone(),
                title: format!("{:?}", kind),
                id: local_id_counter,
            };
            local_id_counter += 1;
            tab
        };

        let tab1 = next_tab(TabKind::Viewport);
        let tab2 = next_tab(TabKind::Explorer);
        let tab3 = next_tab(TabKind::Properties);

        let mut dock_state = DockState::new(vec![tab1]);
        
        let tree = dock_state.main_surface_mut();
        let [viewport, _inspector] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![tab2],
        );
        let [_viewport, _hierarchy] = tree.split_left(
            viewport,
            0.2,
            vec![tab3],
        );

        Self {
            dock_state,
            tab_id_counter: local_id_counter
        }
    }
}