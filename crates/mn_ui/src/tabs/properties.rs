use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Align, Layout, UiBuilder};
use mn_core::{MonoTab, icons::Icon};

use crate::theme::ThemeResource;

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    let pallete = theme.current();
    // persistent selection key per tab
    let sel_key = egui::Id::new(format!("properties_sidebar_sel_{}", tab.id));
    let mut selected: Icon = ui
        .data_mut(|d| d.get_temp::<Icon>(sel_key))
        .unwrap_or(Icon::TabPropertyTools);

    ui.horizontal(|ui| {
        ui.add_space(6.0);

        // ------ Sidebar ------
        // compact, styled frame
        let sidebar = egui::Frame {
            // inner_margin: egui::Margin::same(6),
            fill: pallete.bg,
            corner_radius: egui::CornerRadius::same(6),
            ..Default::default()
        };

        sidebar.show(ui, |ui| {
            ui.set_width(10.0); // sidebar width
            ui.vertical_centered(|ui| {
                // ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 6.0);

                // icons constant (ordered)
                const ICONS: [Icon; 4] = [
                    Icon::TabPropertyView,
                    Icon::TabPropertyExport,
                    Icon::TabPropertyGraphics,
                    Icon::TabPropertyTools,
                ];

                let image_size = egui::vec2(18.0, 18.0);
                for &icon in ICONS.iter() {
                    let tex_id = icon_textures[&icon];
                    let is_sel = selected == icon;

                    // Use theme-aware fills/strokes rather than hard-coded colors
                    let fill = if is_sel {
                        ui.visuals().selection.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    };
                    let stroke = if is_sel {
                        ui.visuals().selection.stroke
                    } else {
                        ui.visuals().widgets.inactive.bg_stroke
                    };

                    let btn = egui::Button::image((tex_id, image_size))
                        .fill(fill)
                        .stroke(stroke)
                        .frame(true);

                    let resp = ui.add(btn);

                    if resp.clicked() {
                        selected = icon;
                    }
                }
            });
        });

        // ------ Content ------
        // 1. remaining rect after sidebar
        let remaining: egui::Rect = ui.available_rect_before_wrap();

        // 2. reserve that rect in the parent's layout so the cursor advances
        ui.allocate_rect(remaining, egui::Sense::hover());

        // 3. create a child UI occupying the exact rect
        let mut right_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(remaining)
                .layout(Layout::top_down(Align::Min)),
        );

        // 4. paint background across full remaining rect (like an expanding Frame)
        right_ui.painter().rect_filled(remaining, 0.0, right_ui.visuals().window_fill());

        // 5. inset for padding
        let padding = 10.0;
        let inner = remaining.shrink(padding);

        // 6. reserve and create inner child UI for content (constrained to inner)
        right_ui.allocate_rect(inner, egui::Sense::hover());
        let mut inner_ui = right_ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::top_down(Align::Min)),
        );

        // 7. content (simple & readable)
        inner_ui.heading(format!("Properties — {}", tab.title));
        inner_ui.separator();
        inner_ui.vertical(|ui| match selected {
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
        });
    }); // top-level horizontal

    // persist selection during run (not across restarts)
    ui.data_mut(|d| d.insert_temp(sel_key, selected));
}
