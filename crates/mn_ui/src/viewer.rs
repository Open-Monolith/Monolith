use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Rect, TextureId};
use egui_dock::TabViewer;

use crate::{tabs, theme::ThemeResource, widgets::tabs_combobox::tabs_combobox};
use mn_core::{ALL_TAB_KINDS, MonoTab, TabKind};

pub struct MyTabViewer<'a> {
    // CHANGED: Store a map of ID -> Rect
    pub viewports: &'a mut HashMap<u32, Rect>,
    pub icon_textures: &'a HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    pub theme: &'a ThemeResource,
}


impl TabViewer for MyTabViewer<'_> {
    type Tab = MonoTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{:?}", tab.kind).into()
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [true, true] // [horizontal, vertical] - disable horizontal, enable vertical
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // tabs_combobox
        let tab_icons = HashMap


        ui.horizontal(|ui| {
            // show selected icon (or a placeholder space) to the left
            if let Some(tex) = selected_tex {
                ui.add(egui::Image::new((tex, icon_size)));
            } else {
                ui.add_space(icon_size.x);
            }

            // ComboBox: set selected_text to empty so only the icon is visible in the closed state.
            egui::ComboBox::from_id_salt("tab_kind_selector")
                .selected_text("") // empty so the icon to the left is the visible selector
                .show_ui(ui, |ui| {
                    for (value, label, tex) in options.iter() {
                        ui.horizontal(|ui| {
                            // create an Image from the (TextureId, Vec2) tuple and add it
                            ui.add(egui::Image::new((*tex, icon_size)));

                            // selectable_value uses the label text; it will set `selected` when clicked
                            // We use the string label as the UI text to the right of the icon.
                            if ui
                                .selectable_value(selected, (*value).clone(), *label)
                                .clicked()
                            {
                                // nothing else required: selectable_value sets `selected`
                            }
                        });
                    }
                });
        });

        // egui::ComboBox::from_id_salt("tab_kind_selector")
        //     .selected_text(format!("{:?}", tab.kind))
        //     .show_ui(ui, |ui| {
        //         for kind in ALL_TAB_KINDS {
        //             if ui
        //                 .selectable_value(&mut tab.kind, kind.clone(), format!("{:?}", kind))
        //                 .clicked()
        //             {
        //                 tab.title = format!("{:?}", kind);
        //             }
        //         }
        //     });

        match tab.kind {
            TabKind::Viewport => {
                tabs::viewport::show(ui, self, tab);
            }
            TabKind::Explorer => {
                tabs::explorer::show(ui, tab, self.icon_textures, self.theme);
            }
            TabKind::Console => {
                tabs::console::show(ui, tab);
            }
            TabKind::Properties => {
                tabs::properties::show(ui, tab, self.icon_textures, self.theme);
            }
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab.kind, TabKind::Viewport)
    }
}
