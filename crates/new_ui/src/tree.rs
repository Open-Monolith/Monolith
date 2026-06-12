use bevy_egui::egui;
use egui_tiles::{Behavior, TileId, UiResponse};

use new_core::pane_kind::PaneKind;
use new_core::Pane;

pub struct TreeBehavior;

impl Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane.kind {
            PaneKind::Viewport => "Viewport".into(),
            PaneKind::Properties => "Properties".into(),
            PaneKind::Console => "Console".into(),
            _ => "Pane".into(),
        }
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        let title = match pane.kind {
            PaneKind::Viewport => "Viewport",
            PaneKind::Properties => "Properties",
            PaneKind::Console => "Console",
            _ => "Pane",
        };
        if pane_header(ui, title) {
            return UiResponse::DragStarted;
        }

        ui.separator();

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
            all_panes_must_have_tabs: false,
            prune_single_child_tabs: true,
            prune_empty_tabs: true,
            prune_empty_containers: true,
            prune_single_child_containers: true,
            join_nested_linear_containers: true,
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

fn pane_header(ui: &mut egui::Ui, title: &str) -> bool {
    let height = 24.0;
    let width = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click_and_drag());

    ui.painter().rect_filled(rect, 0.0, ui.visuals().panel_fill);

    ui.painter().text(
        rect.left_center() + egui::vec2(8.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        egui::TextStyle::Button.resolve(ui.style()),
        ui.visuals().text_color(),
    );

    if response.hovered() {
        ui.ctx().set_cursor_icon(if response.dragged() {
            egui::CursorIcon::Grabbing
        } else {
            egui::CursorIcon::Grab
        });
    }

    response.drag_started()
}