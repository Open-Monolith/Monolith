use bevy::{
    prelude::*,
    math::CompassOctant
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Viewport,
    Inspector,
    Hierarchy,
    Assets,
}

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
    pub viewport_logical: Option<(f32, f32, f32, f32)>,
}

impl DockData {
    pub fn clear_frame(&mut self) {
        self.viewport_logical = None;
    }

    pub fn set_viewport_from_logical(&mut self, left_top: (f32, f32), size: (f32, f32)) {
        self.viewport_logical = Some((left_top.0, left_top.1, size.0, size.1));
    } 
}
