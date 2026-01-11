use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Align, Layout, Response, TextureId};
use mn_core::{MonoTab, icons::Icon};
use crate::theme::ThemeResource;

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    let pallete = theme.current();
    let sel_key = egui::Id::new(format!("properties_sidebar_sel_{}", tab.id));
    let mut selected: Icon = ui
        .data_mut(|d| d.get_temp::<Icon>(sel_key))
        .unwrap_or(Icon::TabPropertyTools);

    // Get full available height for sidebar
    
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0; // Remove gap between sidebar and content
        
        // ------ Sidebar ------
        let sidebar = egui::Frame {
            fill: pallete.bg,
            corner_radius: egui::CornerRadius::same(0),
            inner_margin: egui::Margin::ZERO,
            ..Default::default()
        };

        sidebar.show(ui, |ui| {
            ui.set_min_height(ui.available_height()); // Expand to full height
            ui.set_max_width(18. + 3.); // Constrain width to icon size + padding
            
            ui.vertical_centered(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 6.0);
                ui.style_mut().spacing.item_spacing = egui::Vec2 { x: (0.), y: (0.) };

                const ICONS: [Icon; 10] = [
                    Icon::TabPropertyView,
                    Icon::TabPropertyExport,
                    Icon::TabPropertyGraphics,
                    Icon::TabPropertyTools,
                    Icon::Separator,
                    Icon::TabPropertyConstrait,
                    Icon::TabPropertyMaterial,
                    Icon::TabPropertyModel,
                    Icon::TabPropertyModifiers,
                    Icon::TabPropertyParameters
                ];

                let image_size = egui::vec2(18.0, 18.0);
                for &icon in ICONS.iter() {
                    if icon == Icon::Separator {
                        ui.separator();
                        continue;
                    }
                    let tex_id: TextureId = icon_textures[&icon];
                    let is_sel = selected == icon;
                    
                    let btn: egui::Button = egui::Button::image((tex_id, image_size))
                        .fill(if is_sel { pallete.panel } else { pallete.bg })
                        .stroke(egui::Stroke::NONE)
                        .frame(true);

                    if ui.add(btn).clicked() {
                        selected = icon;
                    }
                }
            });
        });

        // ------ Content ------
        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.heading(format!("Properties — {}", tab.title));
            });
            
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.vertical(|ui| {
                    match selected {
                        Icon::TabPropertyExport => {
                            ui.label("Export settings");
                            ui.label("Value: ...");
                        }
                        Icon::TabPropertyTools => {
                            ui.label("Tool options");
                            ui.label("Value: ...");
                        }
                        Icon::TabPropertyView => {
                            ui.label("Viewport settings");
                            ui.label("Value: ...");
                        }
                        Icon::TabPropertyGraphics => {
                            ui.label("Graphics settings");
                            ui.label("Value: ...");
                        }
                        _ => {
                            ui.label("No property selected");
                        }
                    }
                });
            });
        });
    });

    ui.data_mut(|d| d.insert_temp(sel_key, selected));
}