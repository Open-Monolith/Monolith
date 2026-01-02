use bevy::math::CompassOctant;
use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui, EguiContexts,
};
use egui_dock::DockArea;

use crate::AppWindowCommand;
use crate::{dock_state::DockStateResource, viewer::MyTabViewer};
use crate::widgets::{menubar};


pub fn ui_system(
    mut contexts: EguiContexts,
    mut dock_state_res: ResMut<DockStateResource>, 
    mut dock_data: ResMut<mn_core::DockData>,
    _window: Single<&mut Window, With<PrimaryWindow>>,
    mut appwindow_writer: MessageWriter<AppWindowCommand>, // system param
    ) {


    let Ok(ctx) = contexts.ctx_mut() else { return };
    let screen_r = ctx.viewport_rect();
    if screen_r.width() < 50.0 || screen_r.height() < 50.0 {
        return;
    }
    dock_data.clear_frame();
    
    let mut viewport_rect: Option<egui::Rect> = None;
    draw_resize_borders(ctx, &mut appwindow_writer);

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
                menubar::menu_bar(ctx, ui, appwindow_writer)
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

fn draw_resize_borders(ctx: &egui::Context, writer: &mut MessageWriter<AppWindowCommand>) {
    let rect = ctx.viewport_rect(); 
    let thickness = 6.0; 
    let c_size = 12.0;

    let mut handle = |id: &'static str, cursor: egui::CursorIcon, area: egui::Rect, octant: CompassOctant| {
        egui::Area::new(egui::Id::new(id))
            .fixed_pos(area.left_top())
            // REMOVED: .fixed_size() (Does not exist on Area)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let response = ui.allocate_response(area.size(), egui::Sense::drag());
                
                if response.hovered() {
                    ui.ctx().set_cursor_icon(cursor);
                }
                
                // Use .write() as mandated for Bevy 0.17+
                if response.drag_started() {
                    writer.write(AppWindowCommand::StartResize(octant));
                }
            });
    };

    // --- 1. SIDES ---
    handle("n", egui::CursorIcon::ResizeNorth, 
        egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), thickness)), 
        CompassOctant::North);
        
    handle("s", egui::CursorIcon::ResizeSouth, 
        egui::Rect::from_min_size(egui::pos2(rect.min.x, rect.max.y - thickness), egui::vec2(rect.width(), thickness)), 
        CompassOctant::South);

    handle("w", egui::CursorIcon::ResizeWest, 
        egui::Rect::from_min_size(rect.min, egui::vec2(thickness, rect.height())), 
        CompassOctant::West);

    handle("e", egui::CursorIcon::ResizeEast, 
        egui::Rect::from_min_size(egui::pos2(rect.max.x - thickness, rect.min.y), egui::vec2(thickness, rect.height())), 
        CompassOctant::East);

    // --- 2. CORNERS ---
    handle("nw", egui::CursorIcon::ResizeNorthWest, 
        egui::Rect::from_min_size(rect.min, egui::vec2(c_size, c_size)), 
        CompassOctant::NorthWest);

    handle("ne", egui::CursorIcon::ResizeNorthEast, 
        egui::Rect::from_min_size(egui::pos2(rect.max.x - c_size, rect.min.y), egui::vec2(c_size, c_size)), 
        CompassOctant::NorthEast);

    handle("sw", egui::CursorIcon::ResizeSouthWest, 
        egui::Rect::from_min_size(egui::pos2(rect.min.x, rect.max.y - c_size), egui::vec2(c_size, c_size)), 
        CompassOctant::SouthWest);

    handle("se", egui::CursorIcon::ResizeSouthEast, 
        egui::Rect::from_min_size(egui::pos2(rect.max.x - c_size, rect.max.y - c_size), egui::vec2(c_size, c_size)), 
        CompassOctant::SouthEast);
}