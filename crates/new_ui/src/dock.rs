use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};
use std::collections::HashMap;

use new_core::{DockTree, Pane, PaneKind, UiState, VisibleViewports};

pub fn setup_dock(mut commands: Commands) {
    let mut tiles = Tiles::<Pane>::default();

    let console = tiles.insert_pane(Pane { id: 1, kind: PaneKind::Console });
    let properties = tiles.insert_pane(Pane { id: 2, kind: PaneKind::Properties });
    let viewport = tiles.insert_pane(Pane { id: 3, kind: PaneKind::Viewport });

    let root = tiles.insert_tab_tile(vec![console, properties, viewport]);

    commands.insert_resource(DockTree {
        tree: Tree::new("monolith_tree", root, tiles),
    });
    commands.insert_resource(UiState {
        console_filter: "hello".to_owned(),
        props_enabled: true,
    });
    commands.insert_resource(VisibleViewports::default());
}

pub fn dock_ui_system(
    mut contexts: EguiContexts,
    mut dock: ResMut<DockTree>,
    mut ui_state: ResMut<UiState>,
    mut visible_viewports: ResMut<VisibleViewports>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // When a tile is dropped into an existing Tabs container, egui_tiles adds it
    // as a child but does NOT change the active tab. pane_ui is never called for
    // the new tile, so its rect is never written and nothing updates.
    // We detect new children by diffing snapshots and call set_active ourselves.
    let all_tile_ids: Vec<TileId> = dock.tree.tiles.tile_ids().collect();
    let prev_tab_children: HashMap<TileId, Vec<TileId>> = all_tile_ids
        .iter()
        .filter_map(|&tid| {
            if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
                dock.tree.tiles.get(tid)
            {
                Some((tid, tabs.children.clone()))
            } else {
                None
            }
        })
        .collect();

    let mut frame_viewports = HashMap::new();
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let mut behavior = TreeBehavior {
                ui_state: &mut ui_state,
                visible_viewports: &mut frame_viewports,
            };
            dock.tree.ui(&mut behavior, ui);
        });

    // ── Auto-activate any tile dropped into an existing tab container ──────────
    // Collect first so we can then mutate without fighting the borrow checker.
    let current_tile_ids: Vec<TileId> = dock.tree.tiles.tile_ids().collect();
    let to_activate: Vec<(TileId, TileId)> = current_tile_ids
        .iter()
        .filter_map(|&container_tid| {
            let egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs)) =
                dock.tree.tiles.get(container_tid)?
            else {
                return None;
            };
            let prev = prev_tab_children.get(&container_tid)?;
            // Find the first child that didn't exist before the UI call
            tabs.children
                .iter()
                .find(|&&c| !prev.contains(&c))
                .map(|&new_child| (container_tid, new_child))
        })
        .collect();

    for (container_tid, new_child) in to_activate {
        if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
            dock.tree.tiles.get_mut(container_tid)
        {
            tabs.set_active(new_child);
        }
    }

    visible_viewports.rects = frame_viewports;
}


struct TreeBehavior<'a> {
    ui_state: &'a mut UiState,
    visible_viewports: &'a mut HashMap<u32, egui::Rect>,
}

impl Behavior<Pane> for TreeBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane.kind {
            PaneKind::Console => "Console".into(),
            PaneKind::Properties => "Properties".into(),
            PaneKind::Viewport => "Viewport".into(),
        }
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

        let fill = if state.active {
            ui.visuals().panel_fill
        } else {
            egui::Color32::TRANSPARENT
        };

        let frame = egui::Frame::NONE
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
            PaneKind::Console => {
                paint_opaque_pane_background(ui);
                ui.heading("Console");
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.text_edit_singleline(&mut self.ui_state.console_filter);
                });
            }
            PaneKind::Properties => {
                paint_opaque_pane_background(ui);
                ui.heading("Properties");
                ui.checkbox(&mut self.ui_state.props_enabled, "Enabled");
            }
            PaneKind::Viewport => {
                let rect = ui.max_rect();
                self.visible_viewports.insert(pane.id, rect);
                let _ = ui.allocate_rect(rect, egui::Sense::hover());
            }
        }
        UiResponse::None
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

fn paint_opaque_pane_background(ui: &egui::Ui) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);
}