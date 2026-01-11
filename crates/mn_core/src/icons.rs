use bevy::prelude::*;
use std::sync::LazyLock;
use bevy::platform::collections::HashMap;

#[derive(Resource, Default)]
pub struct IconTextures {
    pub textures: HashMap<Icon, bevy_egui::egui::TextureId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Separator,

    WindowClose,
    WindowMaximize,
    WindowMinimize,

    TabPropertyView,
    TabPropertyGraphics,
    TabPropertyExport,
    TabPropertyTools,

    TabPropertyConstrait,
    TabPropertyMaterial,
    TabPropertyModel,
    TabPropertyModifiers,
    TabPropertyParameters
}

pub static ICON_PATHS: LazyLock<HashMap<Icon, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (Icon::WindowClose, "ui/icons/windows_controls/close/close_dark.png"),
        (Icon::WindowMinimize, "ui/icons/windows_controls/minimize/minimize_dark.png"),
        (Icon::WindowMaximize, "ui/icons/windows_controls/maximize/maximize_dark.png"),

        (Icon::TabPropertyView, "ui/icons/tab_properties/export/export_dark.png"),
        (Icon::TabPropertyGraphics, "ui/icons/tab_properties/graphic/graphic_dark.png"),
        (Icon::TabPropertyExport, "ui/icons/tab_properties/tool/tool_dark.png"),
        (Icon::TabPropertyTools, "ui/icons/tab_properties/view/view_dark.png"),

        (Icon::TabPropertyConstrait, "ui/icons/tab_properties/constraint/constraint_dark.png"),
        (Icon::TabPropertyMaterial, "ui/icons/tab_properties/material/material_dark.png"),
        (Icon::TabPropertyModel, "ui/icons/tab_properties/model/model_dark.png"),
        (Icon::TabPropertyModifiers, "ui/icons/tab_properties/modifiers/modifiers_dark.png"),
        (Icon::TabPropertyParameters, "ui/icons/tab_properties/parameters/parameters_dark.png"),

    ])
});


