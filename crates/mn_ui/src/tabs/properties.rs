use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, CollapsingHeader};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::vertical_tab::icon_sidebar_panel;
use crate::widgets::collapsible::{property_section, property_row};

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
        Icon::TabPropertyModel,
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
        Icon::TabPropertyModel,
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
                Icon::TabPropertyModel => {
                    // Frame with padding
                    property_section(ui, theme,
                        "Type",
                        format!("grid_type_{}", tab.id),
                        |ui, w | {
                            property_row(ui, w, "Type", egui::DragValue::new(&mut 1).speed(1.0));
                            property_row(ui, w, "Typssse", egui::DragValue::new(&mut 1).speed(1.0));
                            property_row(ui, w, "Tysssspe", egui::DragValue::new(&mut 1).speed(1.0));
                        }
                    );
                    
                }
                other => {
                    ui.label(format!("Not implemented: {:?}", other));
                }
            }
        },
    );
}
