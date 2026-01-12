use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, CollapsingHeader};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::vertical_tab::icon_sidebar_panel;

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
                    egui::Frame::NONE
                        .fill(palette.property)
                        .inner_margin(egui::Margin::same(6))
                        .outer_margin(egui::Margin::ZERO)
                        .corner_radius(4.0)
                        .show(ui, |ui| {
                            // CRITICAL FIX 2: Calculate widths INSIDE Frame.show()
                            // Now ui.available_width() returns the correct width AFTER inner_margin
                            let available = ui.available_size().x;

                            // Column percentages
                            let col1_percent = 0.45;
                            let col2_percent = 0.55;

                            let col1_w = available * col1_percent;
                            let col2_w = available * col2_percent;
                            let row_h = ui.spacing().interact_size.y;

                            ui.set_width(col1_w + col2_w);

                            print!("{:?} {:?}\n", available, col1_w + col2_w);

                            CollapsingHeader::new("Type")
                                .default_open(false)
                                .show(ui, |ui| {
                                    let grid_id = format!("grid_type_{}", tab.id);

                                    egui::Grid::new(grid_id)
                                        .num_columns(2)
                                        .min_col_width(0.0) // Allow columns to be sized by widgets
                                        .show(ui, |ui| {
                                            // Row 1: Type
                                            ui.add_sized(
                                                        [col1_w-10., row_h],
                                                        egui::Label::new("Type"),
                                                    );

                                            ui.add_sized(
                                                [col2_w-10., row_h],
                                                egui::DragValue::new(&mut 1).speed(1.0),
                                            );
                                            
                                            ui.end_row();

                                        });
                                });
                        });
                }
                other => {
                    ui.label(format!("Not implemented: {:?}", other));
                }
            }
        },
    );
}
