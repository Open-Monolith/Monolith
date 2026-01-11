use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self};
use mn_core::{MonoTab, icons::Icon};
use crate::theme::ThemeResource;

use crate::widgets::vertical_tab::icon_sidebar_panel; // Import the function above

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    // 1. Define the icons you want for THIS specific tab
    let my_icons = [
        Icon::TabPropertyView,
        Icon::TabPropertyExport,
        Icon::TabPropertyGraphics,
        Icon::TabPropertyTools,
        Icon::Separator,
        Icon::TabPropertyConstrait,
        Icon::TabPropertyMaterial,
    ];

    // 2. Call the reusable widget
    // We pass `tab.id` as the unique ID source so state is saved per tab
    icon_sidebar_panel(
        ui,
        tab.id, 
        icon_textures,
        theme,
        &my_icons,
        Icon::TabPropertyTools, // Default selection
        |ui, selected_icon| {
            // 3. Define the content logic here
            match selected_icon {
                Icon::TabPropertyExport => {
                    ui.heading("Export Settings");
                    ui.label("Format: .OBJ");
                }
                Icon::TabPropertyTools => {
                    ui.heading("Tools");
                    if ui.button("Clean Mesh").clicked() { /* ... */ }
                }
                Icon::TabPropertyGraphics => {
                    ui.heading("Graphics");
                    ui.label("Shadow Quality: High");
                }
                // Handle others...
                _ => {
                    ui.label(format!("Not implemented: {:?}", selected_icon));
                }
            }
        }
    );
}