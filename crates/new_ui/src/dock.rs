use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_tiles::{TileId, Tiles, Container, Tile, Tree};
use std::collections::HashMap;

use new_core::{DockTree, Pane, UiState, VisibleViewports};
use new_core::pane_kind::PaneKind;

use crate::tree::TreeBehavior;

pub fn setup_dock(mut commands: Commands) {
    let mut tiles = Tiles::<Pane>::default();

    let console = tiles.insert_pane(Pane { id: 1, kind: PaneKind::Console });
    let properties = tiles.insert_pane(Pane { id: 2, kind: PaneKind::Properties });
    let viewport = tiles.insert_pane(Pane { id: 3, kind: PaneKind::Viewport });

    let center = tiles.insert_tab_tile(vec![viewport]);
    let right = tiles.insert_tab_tile(vec![properties, console]);

    let root = tiles.insert_horizontal_tile(vec![center, right]);

    if let Some(Tile::Container(Container::Linear(linear))) = tiles.get_mut(root) {
        linear.shares.set_share(center, 4.0);
        linear.shares.set_share(right, 1.0);
    }

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

    let pointer_busy = ctx.input(|i| {
        i.pointer.any_down() || i.pointer.any_released()
    });

    let prev_tab_children = if pointer_busy {
        Some(collect_tab_children(&dock.tree))
    } else {
        None
    };

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let mut behavior = TreeBehavior {
                ui_state: &mut ui_state,
            };

            dock.tree.ui(&mut behavior, ui);
        });

    if let Some(prev_tab_children) = prev_tab_children {
        let repaired = repair_tabs_after_tree_ui(&mut dock.tree, &prev_tab_children);

        if repaired {
            visible_viewports.rects.clear();
            ctx.request_repaint();
            return;
        }
    }

    let new_viewports = collect_visible_viewports(&dock.tree);

    if visible_viewports.rects != new_viewports {
        visible_viewports.rects = new_viewports;
    }

    if pointer_busy {
        ctx.request_repaint();
    }
}

fn collect_tab_children(tree: &Tree<Pane>) -> HashMap<TileId, Vec<TileId>> {
    tree.tiles
        .tile_ids()
        .filter_map(|tile_id| {
            let Some(Tile::Container(Container::Tabs(tabs))) = tree.tiles.get(tile_id) else {
                return None;
            };

            Some((tile_id, tabs.children.clone()))
        })
        .collect()
}

fn repair_tabs_after_tree_ui(
    tree: &mut Tree<Pane>,
    prev_tab_children: &HashMap<TileId, Vec<TileId>>,
) -> bool {
    let tile_ids: Vec<TileId> = tree.tiles.tile_ids().collect();
    let mut to_activate: Vec<(TileId, TileId)> = Vec::new();

    for container_id in tile_ids {
        let Some(Tile::Container(Container::Tabs(tabs))) = tree.tiles.get(container_id) else {
            continue;
        };

        if tabs.children.is_empty() {
            continue;
        }

        // CASE 1:
        // A pane was dropped into this Tabs container.
        // Your current code already handled this case.
        if let Some(prev_children) = prev_tab_children.get(&container_id) {
            if let Some(&new_child) = tabs
                .children
                .iter()
                .find(|&&child| !prev_children.contains(&child))
            {
                to_activate.push((container_id, new_child));
                continue;
            }
        }

        // CASE 2:
        // The active tab was dragged out/reparented.
        // This is your remaining bug.
        let active_is_bad = match tabs.active {
            Some(active) => !tabs.children.contains(&active),
            None => true,
        };

        if active_is_bad {
            if let Some(&fallback_child) = tabs.children.first() {
                to_activate.push((container_id, fallback_child));
            }
        }
    }

    if to_activate.is_empty() {
        return false;
    }

    for (container_id, child_id) in to_activate {
        if let Some(Tile::Container(Container::Tabs(tabs))) = tree.tiles.get_mut(container_id) {
            tabs.set_active(child_id);
        }
    }

    true
}

pub fn collect_visible_viewports(tree: &Tree<Pane>) -> HashMap<u32, egui::Rect> {
    let mut rects = HashMap::new();

    for (&tile_id, tile) in tree.tiles.iter() {
        let Tile::Pane(pane) = tile else {
            continue;
        };

        if pane.kind != PaneKind::Viewport {
            continue;
        }

        let Some(rect) = tree.tiles.rect(tile_id) else {
            continue;
        };

        rects.insert(pane.id, rect);
    }

    rects
}