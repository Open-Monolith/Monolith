use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, TextureId};
use mn_core::icons::Icon;
use std::hash::Hash;

// use crate::icons::IconUiExt;

/// A reusable sidebar widget that handles selection state and layout.
///
/// * `id_source`: Unique identifier for this widget (to store selection state).
/// * `icons`: The list of icons to display in the sidebar.
/// * `default_selection`: The icon to select if none is stored in memory.
/// * `content_builder`: A closure that takes `(ui, selected_icon)` to render the content area.
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

    // 1. Manage State
    // We create a unique ID based on the source passed in (e.g., tab.id)
    let sel_key = ui.make_persistent_id(id_source);
    let mut selected: Icon = ui
        .data_mut(|d| d.get_temp::<Icon>(sel_key))
        .unwrap_or(default_selection);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        // ------ Sidebar ------
        let sidebar_frame = egui::Frame {
            fill: palette.bg,
            corner_radius: egui::CornerRadius::same(0),
            inner_margin: egui::Margin::ZERO,
            ..Default::default()
        };

        sidebar_frame.show(ui, |ui| {
            ui.set_min_height(ui.available_height());
            ui.set_max_width(18.0 + 12.0); // Icon + padding

            ui.vertical_centered(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(6.0, 6.0);
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

                let image_size = egui::vec2(18.0, 18.0);

                for &icon in icons {
                    if icon == Icon::Separator {
                        ui.separator();
                        continue;
                    }

                    // Handle missing texture gracefully
                    let tex_id = *icon_textures.get(&icon).unwrap_or(&TextureId::default());
                    let is_sel = selected == icon;

                    let img = egui::Image::new((tex_id, image_size))
                        .tint(palette.button);

                    let btn = egui::Button::image(img)
                        .fill(if is_sel { palette.panel } else { palette.bg })
                        .stroke(egui::Stroke::NONE)
                        .frame(true);

                    // let resp = ui.icon_button_tinted(tex_id, image_size, theme, is_sel);
                    if ui.add(btn).clicked() {
                        selected = icon;
                    }
                }
            });
        });

        // ------ Content Area ------
        // We use allocate_ui with available_size to ensure it takes the rest of the space
        // without growing infinitely (fixing your previous horizontal scrollbar issue too).
        egui::Frame::NONE.fill(palette.panel).show(ui, |ui| {
            ui.allocate_ui(ui.available_size(), |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        content_builder(ui, selected);
                    });
                });
            });
        });
    });

    // Save state
    ui.data_mut(|d| d.insert_temp(sel_key, selected));
}
