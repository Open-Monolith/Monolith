use bevy::prelude::*;
use std::sync::LazyLock;
use bevy::platform::collections::HashMap;

#[derive(Resource, Default)]
pub struct IconTextures {
    pub textures: HashMap<Icon, bevy_egui::egui::TextureId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    WindowClose,
    WindowMaximize,
    WindowMinimize,
}

pub static ICON_PATHS: LazyLock<HashMap<Icon, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (Icon::WindowClose, "ui/icons/windows_controls/close/close_dark.png"),
        (Icon::WindowMinimize, "ui/icons/windows_controls/minimize/minimize_dark.png"),
        (Icon::WindowMaximize, "ui/icons/windows_controls/maximize/maximize_dark.png"),
    ])
});


