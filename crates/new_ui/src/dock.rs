use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_tiles::{Container, Tile, TileId, Tiles, Tree};
use std::collections::HashMap;

use new_core::pane_kind::PaneKind;
use new_core::{DockTree, Pane, UiState, VisibleViewports};

use crate::tree::TreeBehavior;

pub fn setup_dock(mut commands: Commands) {
    let mut tiles = Tiles::<Pane>::default();

    let viewport = tiles.insert_pane(Pane {
        id: 1,
        kind: PaneKind::Viewport,
    });

    let properties = tiles.insert_pane(Pane {
        id: 2,
        kind: PaneKind::Properties,
    });

    let console = tiles.insert_pane(Pane {
        id: 3,
        kind: PaneKind::Console,
    });

    let right = tiles.insert_vertical_tile(vec![properties, console]);
    let root = tiles.insert_horizontal_tile(vec![viewport, right]);

    if let Some(Tile::Container(Container::Linear(linear))) = tiles.get_mut(root) {
        linear.shares.set_share(viewport, 4.0);
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
    mut visible_viewports: ResMut<VisibleViewports>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let pointer_busy = ctx.input(|i| i.pointer.any_down() || i.pointer.any_released());

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let mut behavior = TreeBehavior;
            dock.tree.ui(&mut behavior, ui);
        });

    let flattened = flatten_tab_stacks(&mut dock.tree);

    if flattened {
        visible_viewports.rects.clear();
        ctx.request_repaint();
        return;
    }

    let new_viewports = collect_visible_viewports(&dock.tree);

    if visible_viewports.rects != new_viewports {
        visible_viewports.rects = new_viewports;
    }

    if pointer_busy {
        ctx.request_repaint();
    }
}

fn flatten_tab_stacks(tree: &mut Tree<Pane>) -> bool {
    let tile_ids: Vec<TileId> = tree.tiles.tile_ids().collect();
    let mut changed = false;

    for tile_id in tile_ids {
        let Some(Tile::Container(container)) = tree.tiles.get_mut(tile_id) else {
            continue;
        };

        let Container::Tabs(tabs) = container else {
            continue;
        };

        if tabs.children.len() < 2 {
            continue;
        }

        let children = tabs.children.clone();
        *container = Container::new_vertical(children);
        changed = true;
    }

    changed
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