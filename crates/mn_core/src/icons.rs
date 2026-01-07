use bevy::prelude::*;
use std::sync::LazyLock;
use bevy::platform::collections::HashMap;

#[derive(Resource)]
pub struct Icons {
    // storing handles makes it cheap to call add_image each frame;
    // bevy_egui caches the TextureId for a handle.
    pub handles: HashMap<Icon, Handle<Image>>,
}

impl Icons {
    pub fn get_handle(&self, key: Icon) -> Option<&Handle<Image>> {
        self.handles.get(&key)
    }
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


