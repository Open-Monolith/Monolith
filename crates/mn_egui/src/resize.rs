use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy_egui::egui;

use mn_core::AppWindowCommand;

pub (crate) fn draw_resize_borders(ctx: &egui::Context, writer: &mut MessageWriter<AppWindowCommand>) {
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