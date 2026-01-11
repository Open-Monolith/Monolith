use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::vertical_tab::icon_sidebar_panel; // Import the function above

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    let palette = theme.current();
    // 1. Define the icons you want for THIS specific tab
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

    // 2. Call the reusable widget
    // We pass `tab.id` as the unique ID source so state is saved per tab
    icon_sidebar_panel(
        ui,
        tab.id,
        icon_textures,
        theme,
        &my_icons,
        Icon::TabPropertyModel, // Default selection
        |ui, selected_icon| {
            let total_width = ui.available_width();
            let col_width = (total_width / 2.0) - 10.0;

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
                Icon::TabPropertyModel => {
                    ui.label("Shadow Quality: High");

                    egui::Frame::NONE
                        .fill(palette.property) 
                        .inner_margin(4.0)
                        .outer_margin(4.0)
                        .corner_radius(4.0)
                        .show(ui, |ui| {

                            ui.set_width(ui.available_width());

                            egui::CollapsingHeader::new("Type")
                                .default_open(true)
                                .show(ui, |ui| {

                                    egui::Grid::new("unique_grid_type")
                                        .num_columns(2)
                                        .spacing([40., 40.])
                                        .striped(true)
                                        .min_col_width(col_width)
                                        .max_col_width(col_width)
                                        .show(ui, |ui| {
                                            // Column 1
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label("Type");
                                                },
                                            );

                                            // Column 2
                                            ui.add(egui::DragValue::new(&mut 2));
                                            ui.end_row();
                                        });
                                });
                        });
                }
                _ => {
                    ui.label(format!("Not implemented: {:?}", selected_icon));
                }
            }
        },
    );
}
