use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, CollapsingHeader};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::sidebar_panel::icon_sidebar_panel;
use crate::tabs::properties_tabs::spatial_tab;


pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    let palette = theme.current();

    let my_icons = [
        Icon::TabPropertyTools,
        Icon::TabPropertyView,
        Icon::TabPropertyGraphics,
        Icon::TabPropertyExport,
        Icon::Separator,
        Icon::TabPropertySpatial,
        Icon::TabPropertyParameters,
        Icon::TabPropertyModifiers,
        Icon::TabPropertyConstrait,
        Icon::TabPropertyMaterial,
    ];

    icon_sidebar_panel(
        ui,
        tab.id,
        icon_textures,
        theme,
        &my_icons,
        Icon::TabPropertySpatial,
        |ui, selected_icon| {
            match selected_icon {
                Icon::TabPropertyExport => {
                    ui.heading("Export Settings");
                    ui.label("Format: .OBJ");
                }
                Icon::TabPropertyTools => {
                    ui.heading("Tools");
                if ui.button("Clean Mesh").clicked() {}
                }
                Icon::TabPropertySpatial => spatial_tab::show(ui, tab, theme),
                other => {
                    ui.label(format!("Not implemented: {:?}", other));
                }
            }
        },
    );
}
