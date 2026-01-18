use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Rect, TextureId, Vec2};
use egui_dock::TabViewer;

use crate::{tabs, theme::ThemeResource, widgets::tabs_combobox::tabs_combobox};
use mn_core::{ALL_TAB_KINDS, MonoTab, TabKind, icons::Icon};

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
        // helper returns Option<TextureId> (owned) not &TextureId
        let tabkind_to_tex = |kind: &TabKind| -> Option<TextureId> {
            match kind {
                TabKind::Viewport => self.icon_textures.get(&Icon::TabViewport).copied(),
                TabKind::Explorer => self.icon_textures.get(&Icon::TabExplorer).copied(),
                TabKind::Console => self.icon_textures.get(&Icon::TabConsole).copied(),
                TabKind::Properties => self.icon_textures.get(&Icon::TabProperty).copied(),
            }
        };

        let icon_size = Vec2::splat(20.0);

        // clone current selection into a local mutable variable.
        // This prevents borrowing `tab.kind` mutably for the duration of UI closures.
        let mut current = tab.kind.clone();

        // use the *current* to draw the closed-state icon (no borrow of tab.kind)
        ui.horizontal(|ui| {
            if let Some(tex) = tabkind_to_tex(&current) {
                ui.add(egui::Image::new((tex, icon_size)));
            } else {
                ui.add_space(icon_size.x);
            }

            let combo_id = ("tab_kind_selector", tab.id);

            egui::ComboBox::from_id_salt(combo_id)
                .selected_text("") // empty — icon on the left is visible
                .icon(|_ui, _rect, _visuals: &egui::style::WidgetVisuals, _is_open| {}) // hide default arrow if you like
                .show_ui(ui, |ui| {
                    for kind in ALL_TAB_KINDS.iter() {
                        let label = format!("{:?}", kind);

                        ui.horizontal(|ui| {
                            if let Some(tex) = tabkind_to_tex(kind) {
                                ui.add(egui::Image::new((tex, icon_size)));
                                ui.add_space(6.0);
                            } else {
                                ui.add_space(icon_size.x + 6.0);
                            }

                            // mutate `current` (local) — NOT `tab.kind`
                            ui.selectable_value(&mut current, kind.clone(), label);
                        });
                    }
                });
        });

        // after the UI closures return, commit change back to `tab` if user selected a new kind
        if current != tab.kind {
            tab.kind = current;
            tab.title = format!("{:?}", tab.kind);
        }

        // now use tab.kind safely for the rest of your UI
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
