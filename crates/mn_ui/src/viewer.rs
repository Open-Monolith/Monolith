use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;
use strum::IntoEnumIterator;

use mn_core::{MonoTab, TabKind, icons::Icon};
use crate::{tabs, theme::ThemeResource};

pub struct MyTabViewer<'a> {
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
        tab.kind.to_string().into()
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [false, true]
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        

        ui.set_min_width(250.0);
        
        let tabkind_icon = |kind: &TabKind| -> &Icon {
            match kind {
                TabKind::Viewport => &Icon::TabViewport,
                TabKind::Explorer => &Icon::TabExplorer,
                TabKind::Console => &Icon::TabConsole,
                TabKind::Properties => &Icon::TabProperty,
                TabKind::AssetBrowser => &Icon::TabAssetBrowser,
            }
        };

        let mut current = tab.kind.clone();
        let image_size = egui::vec2(18.0, 18.0);
        let select_button = ui.add(
            egui::Button::image((
            self.icon_textures[tabkind_icon(&tab.kind)],
            image_size,
            ))
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
        );

        dropdown_popup(&select_button, format!("basic_dropdown_{}", tab.id), |ui| {
            for kind in TabKind::iter() {
                if ui
                    .add(egui::Button::image_and_text(
                        (self.icon_textures[tabkind_icon(&kind)], image_size),
                        kind.to_string(),
                    )
                    )
                    .clicked()
                {
                    current = kind;
                }
            }
        });

        if current != tab.kind {
            tab.kind = current;
        }

        match tab.kind {
            TabKind::Viewport => tabs::viewport::show(ui, self, tab),
            TabKind::Explorer => tabs::explorer::show(ui, tab, self.icon_textures, self.theme),
            TabKind::Console => tabs::console::show(ui, tab),
            TabKind::Properties => tabs::properties::show(ui, tab, self.icon_textures, self.theme),
            TabKind::AssetBrowser => tabs::console::show(ui, tab),
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab.kind, TabKind::Viewport)
    }
}

pub fn dropdown_popup<F>(
    button_response: &egui::Response,
    popup_id: impl Into<egui::Id>,
    content_builder: impl FnOnce(&mut egui::Ui) -> F,
) -> Option<F> {
    egui::Popup::menu(button_response)
        .id(popup_id.into())
        .show(content_builder)
        .map(|response| response.inner)
}
