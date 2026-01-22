use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::sidebar_panel::sidebar_panel;
use crate::tabs::properties_tabs::{
    spatial_tab,
    information_tab,
    parameters_tab,
    graphics_tab,
    modifiers_tab,
    collaboration_tab
};


pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    // let palette = theme.current();

    let my_icons = [
        Icon::TabPropertyView,
        Icon::TabPropertyExport,
        Icon::Separator,
        Icon::TabPropertySpatial,
        Icon::TabPropertyInformation,
        Icon::TabPropertyParameters,
        Icon::TabPropertyGraphics,
        Icon::TabPropertyModifiers,
        Icon::TabPropertyMaterial,
        Icon::TabPropertyCollaboration,
    ];


    sidebar_panel(
        ui,
        tab.id,
        icon_textures,
        theme,
        &my_icons,
        "TabProperty",
        Icon::TabPropertySpatial,
        |ui, selected_icon| {
            match selected_icon {
                Icon::TabPropertySpatial => spatial_tab::show(ui, tab, theme),
                Icon::TabPropertyInformation => information_tab::show(ui, tab, theme),
                Icon::TabPropertyParameters => parameters_tab::show(ui, tab, theme),
                Icon::TabPropertyGraphics => graphics_tab::show(ui, tab, theme),
                Icon::TabPropertyModifiers => modifiers_tab::show(ui, tab, theme),
                Icon::TabPropertyCollaboration => collaboration_tab::show(ui, tab, theme),
                other => {
                    ui.label(format!("Not implemented: {:?}", other));
                }
            }
        },
    );
}
