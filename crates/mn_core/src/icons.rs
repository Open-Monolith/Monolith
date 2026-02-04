use bevy::prelude::*;
use bevy::platform::collections::HashMap;
use strum_macros::{VariantArray};

#[derive(Resource, Default)]
pub struct IconTextures {
    pub textures: HashMap<Icon, bevy_egui::egui::TextureId>,
}

#[derive(VariantArray, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    TabAssetBrowser,

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

    // Tools Modify
    ToolModifyAlign,
    ToolModifyCut,
    ToolModifyMirror,
    ToolModifyMove,
    ToolModifyRotate,
    ToolModifySelect,
    ToolModifyTrim,

    // Tools Architect
    ToolArchitectWall,
}

#[inline]
pub const fn icon_path(icon: &Icon) -> Option<&'static str> {
    Some(match icon {
        Icon::WindowClose => "ui/icons/windows_controls/close/close.png",
        Icon::WindowMinimize => "ui/icons/windows_controls/minimize/minimize.png",
        Icon::WindowMaximize => "ui/icons/windows_controls/maximize/maximize.png",
        
        // Tab Property
        Icon::TabPropertyView => "ui/icons/tab_properties/view/view.png",
        Icon::TabPropertyExport => "ui/icons/tab_properties/export/export.png",

        // Tab Object Property
        Icon::TabPropertySpatial => "ui/icons/tab_properties/spatial/spatial.png",
        Icon::TabPropertyInformation => "ui/icons/tab_properties/information/information.png",
        Icon::TabPropertyParameters => "ui/icons/tab_properties/parameters/parameters.png",
        Icon::TabPropertyGraphics => "ui/icons/tab_properties/graphic/graphic.png",
        Icon::TabPropertyModifiers => "ui/icons/tab_properties/modifiers/modifiers.png",
        Icon::TabPropertyMaterial => "ui/icons/tab_properties/material/material.png",
        Icon::TabPropertyCollaboration => "ui/icons/tab_properties/collaboration/collaboration.png",

        // Tab Explorer
        Icon::TabExplorerAssets => "ui/icons/tab_explorer/assets/assets.png",
        Icon::TabExplorerGroups => "ui/icons/tab_explorer/groups/groups.png",
        Icon::TabExplorerSchedules => "ui/icons/tab_explorer/schedules/schedules.png",
        Icon::TabExplorerSheets => "ui/icons/tab_explorer/sheets/sheets.png",
        Icon::TabExplorerViews => "ui/icons/tab_explorer/views/views.png",

        // Tab
        Icon::TabConsole => "ui/icons/tab/console/console.png",
        Icon::TabExplorer => "ui/icons/tab/explorer/explorer.png",
        Icon::TabProperty => "ui/icons/tab/property/property.png",
        Icon::TabViewport => "ui/icons/tab/viewport/viewport.png",
        Icon::TabAssetBrowser => "ui/icons/tab/asset_browser/asset_browser.png",

        // Tool Modify
        Icon::ToolModifyAlign => "ui/icons/tool_modify/align/align.png",
        Icon::ToolModifyCut => "ui/icons/tool_modify/cut/cut.png",
        Icon::ToolModifyMirror => "ui/icons/tool_modify/mirror/mirror.png",
        Icon::ToolModifyMove => "ui/icons/tool_modify/move/move.png",
        Icon::ToolModifyRotate => "ui/icons/tool_modify/rotate/rotate.png",
        Icon::ToolModifySelect => "ui/icons/tool_modify/select/select.png",
        Icon::ToolModifyTrim => "ui/icons/tool_modify/trim/trim.png",
        
        // Tool Architect
        Icon::ToolArchitectWall => "ui/icons/tool_architect/wall/wall.png",

        Icon::Separator => return None,
    })
}
