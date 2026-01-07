use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;

use mn_core::{MonoTab, TabKind, ALL_TAB_KINDS};
use crate::tabs;

pub struct MyTabViewer<'a> {
    // CHANGED: Store a map of ID -> Rect
    pub viewports: &'a mut HashMap<u32, Rect>,
}

impl TabViewer for MyTabViewer<'_> {
    type Tab = MonoTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{:?}", tab.kind).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {

        egui::ComboBox::from_id_salt("tab_kind_selector")
            .selected_text(format!("{:?}", tab.kind))
            .show_ui(ui, |ui| {
                for kind in ALL_TAB_KINDS {
                    if ui
                        .selectable_value(&mut tab.kind, kind.clone(), format!("{:?}", kind))
                        .clicked()
                    {
                        tab.title = format!("{:?}", kind);
                    }
                }
            });

        match tab.kind {
            TabKind::Viewport => {
                tabs::viewport::show(ui, self, tab);
            }
            TabKind::Explorer => {
                tabs::explorer::show(ui, tab);
            }
            TabKind::Console => {
                tabs::console::show(ui, tab);
            }
            TabKind::Properties => {
                tabs::properties::show(ui, tab);
            }
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab.kind, TabKind::Viewport)
    }
}