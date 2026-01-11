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

    // Properties
    TabPropertyView,
    TabPropertyGraphics,
    TabPropertyExport,
    TabPropertyTools,

    TabPropertyConstrait,
    TabPropertyMaterial,
    TabPropertyModel,
    TabPropertyModifiers,
    TabPropertyParameters,

    // Explorer
    TabExplorerAssets,
    TabExplorerGroups,
    TabExplorerSchedules,
    TabExplorerSheets,
    TabExplorerViews,
}

pub static ICON_PATHS: LazyLock<HashMap<Icon, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (Icon::WindowClose, "ui/icons/windows_controls/close/close.png"),
        (Icon::WindowMinimize, "ui/icons/windows_controls/minimize/minimize.png"),
        (Icon::WindowMaximize, "ui/icons/windows_controls/maximize/maximize.png"),

        // Properties
        (Icon::TabPropertyView, "ui/icons/tab_properties/export/export.png"),
        (Icon::TabPropertyGraphics, "ui/icons/tab_properties/graphic/graphic.png"),
        (Icon::TabPropertyExport, "ui/icons/tab_properties/tool/tool.png"),
        (Icon::TabPropertyTools, "ui/icons/tab_properties/view/view.png"),

        (Icon::TabPropertyConstrait, "ui/icons/tab_properties/constraint/constraint.png"),
        (Icon::TabPropertyMaterial, "ui/icons/tab_properties/material/material.png"),
        (Icon::TabPropertyModel, "ui/icons/tab_properties/model/model.png"),
        (Icon::TabPropertyModifiers, "ui/icons/tab_properties/modifiers/modifiers.png"),
        (Icon::TabPropertyParameters, "ui/icons/tab_properties/parameters/parameters.png"),

        // Explorer
        (Icon::TabExplorerAssets, "ui/icons/tab_explorer/assets/assets.png"),
        (Icon::TabExplorerGroups, "ui/icons/tab_explorer/groups/groups.png"),
        (Icon::TabExplorerSchedules, "ui/icons/tab_explorer/schedules/schedules.png"),
        (Icon::TabExplorerSheets, "ui/icons/tab_explorer/sheets/sheets.png"),
        (Icon::TabExplorerViews, "ui/icons/tab_explorer/views/views.png"),

    ])
});


