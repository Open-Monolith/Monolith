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

    // Tab
    TabConsole,
    TabProperty,
    TabExplorer,
    TabViewport,

    // Properties
    TabPropertyView,
    TabPropertyExport,
    
    // Object Properties
    TabPropertySpatial,
    TabPropertyInformation,
    TabPropertyParameters,
    TabPropertyGraphics,
    TabPropertyModifiers,
    TabPropertyMaterial,
    TabPropertyCollaboration,
    // TabPropertyConstrait,

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
        (Icon::TabPropertyExport, "ui/icons/tab_properties/tool/tool.png"),

        // Object Properties
        (Icon::TabPropertySpatial, "ui/icons/tab_properties/spatial/spatial.png"),
        (Icon::TabPropertyInformation, "ui/icons/tab_properties/information/information.png"),
        (Icon::TabPropertyParameters, "ui/icons/tab_properties/parameters/parameters.png"),
        (Icon::TabPropertyGraphics, "ui/icons/tab_properties/graphic/graphic.png"),
        (Icon::TabPropertyModifiers, "ui/icons/tab_properties/modifiers/modifiers.png"),
        (Icon::TabPropertyMaterial, "ui/icons/tab_properties/material/material.png"),
        (Icon::TabPropertyCollaboration, "ui/icons/tab_properties/collaboration/collaboration.png"),

        // (Icon::TabPropertyTools, "ui/icons/tab_properties/view/view.png"),
        // (Icon::TabPropertyConstrait, "ui/icons/tab_properties/constraint/constraint.png"),

        // Explorer
        (Icon::TabExplorerAssets, "ui/icons/tab_explorer/assets/assets.png"),
        (Icon::TabExplorerGroups, "ui/icons/tab_explorer/groups/groups.png"),
        (Icon::TabExplorerSchedules, "ui/icons/tab_explorer/schedules/schedules.png"),
        (Icon::TabExplorerSheets, "ui/icons/tab_explorer/sheets/sheets.png"),
        (Icon::TabExplorerViews, "ui/icons/tab_explorer/views/views.png"),

        // Tabs
        (Icon::TabConsole, "ui/icons/tab/console/console.png"),
        (Icon::TabExplorer, "ui/icons/tab/explorer/explorer.png"),
        (Icon::TabProperty, "ui/icons/tab/property/property.png"),
        (Icon::TabViewport, "ui/icons/tab/viewport/viewport.png"),
    ])
});


