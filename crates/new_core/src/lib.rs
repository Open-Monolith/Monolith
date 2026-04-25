use bevy::prelude::*;
use bevy_egui::egui;
use egui_tiles::Tree;
use std::collections::HashMap;


pub mod element;
pub mod elements;
pub mod placement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaneKind {
    Console,
    Properties,
    Viewport,
}

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

