use bevy::prelude::*;
use bevy_egui::egui;
use egui_tiles::{Behavior, TileId, Tiles, UiResponse};
use std::collections::HashMap;

use new_core::{Pane, UiState};
use new_core::pane_kind::PaneKind;



pub struct TreeBehavior<'a> {
    pub ui_state: &'a mut UiState,
}

impl Behavior<Pane> for TreeBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.kind.to_string().into()
    }

    fn tab_ui(
        &mut self,
        tiles: &mut Tiles<Pane>,
        ui: &mut egui::Ui,
        id: egui::Id,
        tile_id: egui_tiles::TileId,
        state: &egui_tiles::TabState,
    ) -> egui::Response {
        let title = self.tab_title_for_tile(tiles, tile_id);

        let text_color = if state.active {
            egui::Color32::WHITE
        } else if state.is_being_dragged {
            egui::Color32::LIGHT_BLUE
        } else {
            egui::Color32::GRAY
        };

        let stroke = if state.active {
            egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill)
        } else {
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
        };

        let fill = ui.visuals().panel_fill;

        let frame = egui::Frame::NONE
            .fill(fill)
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(title.text().to_owned()).color(text_color),
                    )
                    .selectable(false)
                    .sense(egui::Sense::hover()),
                )
            });

        let tab_response = ui.interact(frame.response.rect, id, egui::Sense::click_and_drag());

        if tab_response.hovered() {
            ui.ctx().set_cursor_icon(if state.is_being_dragged {
                egui::CursorIcon::Grabbing
            } else {
                egui::CursorIcon::Grab
            });
        }

        if state.active && !state.is_being_dragged {
            ui.painter().hline(
                frame.response.rect.x_range(),
                frame.response.rect.bottom(),
                egui::Stroke::new(stroke.width + 1.0, fill),
            );
        }

        tab_response    
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        match pane.kind {
            PaneKind::Console => crate::pane::pane_console::show(ui),
            PaneKind::Properties => crate::pane::pane_properties::show(ui),    
            PaneKind::Viewport => crate::pane::pane_viewport::show(ui, pane),
            _ => {}
        }
        UiResponse::None
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }


    fn tab_bar_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        visuals.panel_fill
    }

    fn tab_bar_hline_stroke(&self, visuals: &egui::Visuals) -> egui::Stroke {
        egui::Stroke::new(1.0, visuals.widgets.noninteractive.bg_stroke.color)
    }

    fn preview_dragged_panes(&self) -> bool {
        false
}
}
