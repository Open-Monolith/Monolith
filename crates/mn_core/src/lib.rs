use bevy::{
    math::CompassOctant, platform::collections::HashMap, prelude::*
};
use strum_macros::{EnumIter, Display};

pub mod icons;
pub mod enums;
pub mod element;

#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabKind {
    #[strum(to_string="Viewport")]
    Viewport,

    #[strum(to_string="Explorer")]
    Explorer,

    #[strum(to_string="Properties")]
    Properties,

    #[strum(to_string="Console")]
    Console,

    #[strum(to_string="Asset Browser")]
    AssetBrowser,
}

pub struct MonoTab {
    pub kind: TabKind,
    pub title: String,
    pub id: u32,
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
    pub viewports: HashMap<u32, (f32, f32, f32, f32)>,
}

impl DockData {
    pub fn clear_frame(&mut self) {
        self.viewports.clear();
    }
}
