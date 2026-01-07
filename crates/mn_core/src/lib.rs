use bevy::{
    math::CompassOctant, platform::collections::HashMap, prelude::*
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabKind {
    Viewport,
    Explorer,
    Properties,
    Console,
}

pub struct MonoTab {
    pub kind: TabKind,
    pub title: String,
    pub id: u32,
}

pub const ALL_TAB_KINDS: [TabKind; 4] = [
    TabKind::Viewport,
    TabKind::Explorer,
    TabKind::Properties,
    TabKind::Console,
];



#[derive(Message, Clone)]
pub enum AppWindowCommand {
    Minimize,
    ToggleMaximize,
    Shutdown,
    StartMove,
    StartResize(CompassOctant),
}

#[derive(Resource, Default, Debug)]
pub struct DockData {
    // pub viewport_logical: Option<(f32, f32, f32, f32)>,
    pub viewports: HashMap<u32, (f32, f32, f32, f32)>,
}

impl DockData {
    pub fn clear_frame(&mut self) {
        // self.viewport_logical = None;
        self.viewports.clear();
    }

    // pub fn set_viewport_from_logical(&mut self, left_top: (f32, f32), size: (f32, f32)) {
    //     self.viewport_logical = Some((left_top.0, left_top.1, size.0, size.1));
    // } 
}
