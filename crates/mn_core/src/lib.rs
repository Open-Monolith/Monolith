use bevy::{
    math::CompassOctant, platform::collections::HashMap, prelude::*
};

pub mod icons;

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
    pub viewports: HashMap<u32, (f32, f32, f32, f32)>,
}

impl DockData {
    pub fn clear_frame(&mut self) {
        self.viewports.clear();
    }
}
