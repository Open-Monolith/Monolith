use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self};
use mn_core::{MonoTab, icons::Icon};

use crate::theme::Theme;

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &Theme,
) {
    // Unique key per tab (so different tab ids keep different selections)
    let sel_key = egui::Id::new(format!("properties_sidebar_sel_{}", tab.id));

    // Read current selection from egui temporary memory (default = 1)
    let mut selected: Icon = ui.data_mut(|data| {
        data.get_temp::<Icon>(sel_key)
            .unwrap_or(Icon::TabPropertyTools)
    });

    ui.horizontal(|ui| {
        ui.add_space(4.0);
        // LEFT: sidebar
        const CORNERR: u8 = 5;
        let sidebar_frame = egui::Frame {
            // inner_margin: egui::Margin::same(4),
            fill: ui.visuals().panel_fill,
            corner_radius: egui::CornerRadius {
                nw: (CORNERR),
                ne: (CORNERR),
                sw: (CORNERR),
                se: (CORNERR),
            },
            ..Default::default()
        };
        sidebar_frame.show(ui, |ui| {
            ui.set_width(10.0); // tweak width to taste
            ui.vertical_centered(|ui| {
                // ui.set_width(4.0);

                let icons = [
                    Icon::TabPropertyView,
                    Icon::TabPropertyExport,
                    Icon::TabPropertyGraphics,
                    Icon::TabPropertyTools,
                ];

                for icon in icons {
                    ui.style_mut().spacing.button_padding = egui::vec2(3.0, 3.0);
                    let image_size = egui::vec2(15.0, 15.0);
                    let is_sel: bool = selected == icon;
                    let fill_color: egui::Color32 = if is_sel {
                        theme.panel
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    let tex_id: egui::TextureId = icon_textures[&icon];
                    let image_btn = egui::Button::image((tex_id, image_size))
                        .selected(selected == icon)
                        .fill(fill_color)
                        .corner_radius(egui::CornerRadius::same(0))
                        .frame(true);

                    let resp = ui.add(image_btn);
                    if resp.clicked() {
                        selected = icon;
                    }
                }
                // ui.separator();
            });
        });

        // RIGHT: content area for the selected property
        let content_frame = egui::Frame {
            inner_margin: egui::Margin::same(8),
            fill: ui.visuals().window_fill(),
            ..Default::default()
        };
        content_frame.show(ui, |ui| {
            ui.vertical(|ui| match selected {
                Icon::TabPropertyExport => {
                    ui.label("Property 1");
                    ui.label("Value: .sssssssssssss..");
                }
                Icon::TabPropertyTools => {
                    ui.label("Property 2");
                    ui.label("Value: ...");
                }
                Icon::TabPropertyView => {
                    ui.label("Property 3");
                    ui.label("Value: ...");
                }
                Icon::TabPropertyGraphics => {
                    ui.label("Property 4");
                    ui.label("Value: ...");
                }
                _ => {
                    ui.label("No property selected");
                }
            });
        });
    });

    // Save selection back into egui temporary memory (won't persist across app restarts)
    ui.data_mut(|data| data.insert_temp(sel_key, selected));
}
