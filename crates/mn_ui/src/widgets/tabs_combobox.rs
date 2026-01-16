use bevy_egui::egui::{self, ComboBox, TextureId, Ui, Vec2};

/// Icon + label combo: shows the selected icon (only) when closed.
/// `id_source` should be something convertible into `egui::Id` (e.g. a string or egui::Id).
/// `options` is slice of (value, label, tex_id).
pub fn tabs_combobox<T>(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash,
    options: &[(T, &'static str, TextureId)], // value, label, texture
    selected: &mut T,
    icon_size: Vec2,
) where
    T: Copy + PartialEq + Clone + std::fmt::Debug,
{
    // Find texture for currently selected value (if any)
    let selected_tex: Option<TextureId> = options
        .iter()
        .find(|(v, _, _)| *v == *selected)
        .map(|(_, _, tex)| *tex);

    ui.horizontal(|ui| {
        // show selected icon (or a placeholder space) to the left
        if let Some(tex) = selected_tex {
            ui.add(egui::Image::new((tex, icon_size)));
        } else {
            ui.add_space(icon_size.x);
        }

        // ComboBox: set selected_text to empty so only the icon is visible in the closed state.
        ComboBox::from_id_salt(id_salt)
            .selected_text("") // empty so the icon to the left is the visible selector
            .show_ui(ui, |ui| {
                for (value, label, tex) in options.iter() {
                    ui.horizontal(|ui| {
                        // create an Image from the (TextureId, Vec2) tuple and add it
                        ui.add(egui::Image::new((*tex, icon_size)));

                        // selectable_value uses the label text; it will set `selected` when clicked
                        // We use the string label as the UI text to the right of the icon.
                        if ui.selectable_value(selected, (*value).clone(), *label).clicked() {
                            // nothing else required: selectable_value sets `selected`
                        }
                    });
                }
            });
    });
}
