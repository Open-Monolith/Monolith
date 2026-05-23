use bevy::prelude::*;
use bevy_egui::egui;
use egui_tiles::{TileId, Tree};
use std::collections::HashMap;


pub mod element;
pub mod elements;
pub mod placement;
pub mod pane_kind;

use crate::pane_kind::{
    PaneKind
};

#[derive(Clone, Debug)]
pub struct Pane {
    pub id: u32,
    pub kind: PaneKind,
}

#[derive(Resource)]
pub struct DockTree {
    pub tree: Tree<Pane>,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub console_filter: String,
    pub props_enabled: bool,
}

#[derive(Resource, Default)]
pub struct VisibleViewports {
    pub rects: HashMap<u32, egui::Rect>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GameViewportCamera {
    pub pane_id: u32,
}

