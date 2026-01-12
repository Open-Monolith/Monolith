use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, TextureId};
use mn_core::icons::Icon;
use std::hash::Hash;

const SIDEBAR_WIDTH: f32 = 30.0; // 18px icon + 12px padding

/// A reusable sidebar widget that handles selection state and layout.
pub fn icon_sidebar_panel<F>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    icon_textures: &HashMap<Icon, TextureId>,
    theme: &ThemeResource,
    icons: &[Icon],
    default_selection: Icon,
    content_builder: F,
) where
    F: FnOnce(&mut egui::Ui, Icon),
{
    let palette = theme.current();

    // Manage State
    let sel_key = ui.make_persistent_id(id_source);
    let mut selected: Icon = ui
        .data_mut(|d| d.get_temp::<Icon>(sel_key))
        .unwrap_or(default_selection);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        // ------ Sidebar (FIXED WIDTH) ------
        egui::Frame::NONE.fill(palette.bg).show(ui, |ui| {
            // Set exact width for sidebar
            ui.set_width(SIDEBAR_WIDTH);
            ui.set_min_height(ui.available_height());

            // Use vertical (NOT vertical_centered which takes all horizontal space)
            ui.vertical(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 6.0);
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

                let image_size = egui::vec2(18.0, 18.0);

                for &icon in icons {
                    if icon == Icon::Separator {
                        ui.separator();
                        continue;
                    }

                    let tex_id = *icon_textures.get(&icon).unwrap_or(&TextureId::default());
                    let is_sel = selected == icon;

                    let img = egui::Image::new((tex_id, image_size)).tint(palette.button);
                    let btn = egui::Button::image(img)
                        .fill(if is_sel { palette.panel } else { palette.bg })
                        .stroke(egui::Stroke::NONE)
                        .frame(true);

                    // Center button horizontally
                    let btn_offset = (SIDEBAR_WIDTH - image_size.x - 12.0) / 2.0;
                    if btn_offset > 0.0 {
                        ui.add_space(btn_offset);
                    }

                    if ui.add(btn).clicked() {
                        selected = icon;
                    }
                }
            });
        });

        // ------ Content Area ------

        egui::Frame::NONE
            .fill(palette.panel)
            .inner_margin(egui::Margin {
                left: 10,
                right: 5,
                top: 10,
                bottom: 10,
            })
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    content_builder(ui, selected);
                });
            });
    });

    // Save state
    ui.data_mut(|d| d.insert_temp(sel_key, selected));
}
