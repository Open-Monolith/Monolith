use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, TextureId};
use mn_core::icons::Icon;
use std::hash::Hash;

const SIDEBAR_WIDTH: f32 = 12.0; // 18px icon + 12px padding
const IMAGE_SIZE: egui::Vec2 = egui::Vec2::splat(18.0);

pub fn sidebar_panel<F>(
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
    let sel_key = ui.make_persistent_id(id_source);

    let mut selected: Icon = ui
        .data_mut(|d| d.get_temp::<Icon>(sel_key))
        .unwrap_or(default_selection);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        // ------ Sidebar ------
        egui::Frame::NONE.fill(palette.bg).show(ui, |ui| {
            ui.set_width(SIDEBAR_WIDTH);
            ui.set_min_height(ui.available_height());

            ui.vertical(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 6.0);
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

                for &icon in icons {
                    if icon == Icon::Separator {
                        ui.separator();
                        continue;
                    }

                    let tex_id = *icon_textures.get(&icon).unwrap_or(&TextureId::default());
                    let is_sel = selected == icon;

                    let img = egui::Image::new((tex_id, IMAGE_SIZE)).tint(palette.button);
                    let btn = egui::Button::image(img)
                        .fill(if is_sel { palette.panel } else { palette.bg })
                        .stroke(egui::Stroke::NONE)
                        .corner_radius(egui::CornerRadius {
                            nw: 3,
                            ne: 0,
                            sw: 3,
                            se: 0,
                        })
                        .frame(true);

                    let btn_offset = (SIDEBAR_WIDTH - IMAGE_SIZE.x - 12.0) / 2.0;
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
                left: 5,
                right: 5,
                top: 0,
                bottom: 10,
            })
            .show(ui, |ui| {
                ui.vertical(|ui| {

                    ui.horizontal(|ui| {
                        let tex_id = *icon_textures
                            .get(&selected)
                            .unwrap_or(&TextureId::default());
                        ui.add(egui::Image::new((tex_id, IMAGE_SIZE)).tint(palette.button));
                        ui.label(format!(" {:?}", selected).replace("TabProperty", ""));
                    });
                    content_builder(ui, selected);
                });
            });
    });

    // Save state
    ui.data_mut(|d| d.insert_temp(sel_key, selected));
}
