use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui, EguiContexts,
};
use egui_dock::DockArea;

use crate::{dock_state::DockStateResource, viewer::MyTabViewer};
use crate::widgets::{menubar};


pub fn ui_system(
    mut contexts: EguiContexts,
    mut dock_state_res: ResMut<DockStateResource>, 
    mut dock_data: ResMut<mn_core::DockData>,
    _window: Single<&mut Window, With<PrimaryWindow>>,
) {


    let Ok(ctx) = contexts.ctx_mut() else { return };

    dock_data.clear_frame();
    
    let mut viewport_rect: Option<egui::Rect> = None;


    egui::TopBottomPanel::top("main_menu_bar")
        .show_separator_line(false)
        .frame(egui::Frame {
            inner_margin: egui::Margin::symmetric(15, 8),
            fill: hex_to_color("#181818"),
            stroke: egui::Stroke::NONE,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                file_menu(ctx, ui);
                edit_menu(ctx, ui);
                window_menu(ctx, ui);
                view_menu(ctx, ui);
            });
        });

    DockArea::new(&mut dock_state_res.dock_state)
        .show(ctx, 
            &mut MyTabViewer {
                viewport_out: &mut viewport_rect
            });
    
    if let Some(rect) = viewport_rect {
        let lt = rect.left_top();
        let sz = rect.size();
        dock_data.set_viewport_from_logical((lt.x, lt.y), (sz.x, sz.y));
    }
}

pub fn hex_to_color(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

fn file_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("New Project").clicked() {}
        if ui.button("Open Project...").clicked() {}
        ui.menu_button("Open Recent", |ui| {
            if ui.button("Kahuina.monolith").clicked() {}
            if ui.button("Uptown_mall.monolith").clicked() {}
            if ui.button("Launiu Project.monolith").clicked() {}
        });
        if ui.button("Save").clicked() {}
        if ui.button("Save as...").clicked() {}
        ui.separator();
        ui.menu_button("Import", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("glTF / GLB").clicked() {}
            if ui.button("CAD (.dxf)").clicked() {}
        });
        ui.menu_button("Export", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("Sheet (.pdf)").clicked() {}
        });
        ui.separator();
        if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

fn edit_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Edit", |ui| {
        if ui.button("Undo (Ctrl + Z)").clicked() {}
        if ui.button("Redo (Ctrl + Y)").clicked() {}
        ui.separator();
        if ui.button("Cut (Ctrl + X)").clicked() {}
        if ui.button("Copy (Ctrl + C)").clicked() {}
        if ui.button("Paste (Ctrl + V)").clicked() {}
        if ui.button("Delete (Del)").clicked() {}
        ui.separator();
        ui.menu_button("Select All...", |ui| {
            if ui.button("in Current View").clicked() {}
            if ui.button("in Entire Project").clicked() {}
        });
        if ui.button("Deselect All (Alt + A)").clicked() {}
        ui.separator();
        if ui.button("Commands (Space)").clicked() {}
        if ui.button("Preferences...").clicked() {}
    });
}

fn window_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Window", |ui| {
        ui.menu_button("Workspaces", |ui| {
            if ui.button("Modeling").clicked() {}
            if ui.button("Drafting").clicked() {}
            if ui.button("Forging").clicked() {}
            if ui.button("Rendering").clicked() {}
            if ui.button("Scripting").clicked() {}
            if ui.button("Collaboration").clicked() {}
        });
        ui.separator();
        if ui.button("Scene View").clicked() {}
        if ui.button("Properties").clicked() {}
        if ui.button("File Explorer").clicked() {}
        if ui.button("Scripting").clicked() {}
        if ui.button("Asset Browser").clicked() {}
        if ui.button("Console").clicked() {}
        ui.separator();
        if ui.button("Toggle Fullscreen (F11)").clicked() {}
        if ui.button("Reset Layout").clicked() {}
    });
}

fn view_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("View", |ui| {
        if ui.button("Documentation").clicked() {}
        if ui.button("Keyboard Shortcuts").clicked() {}
        if ui.button("Report a Bug").clicked() {}
        if ui.button("Community").clicked() {}
        ui.separator();
        if ui.button("Developer Tools").clicked() {}
        if ui.button("About Monolith").clicked() {}
    });
}