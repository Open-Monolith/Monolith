use eframe::egui;

use crate::{theme::Theme, widgets::windows};


pub(crate) fn draw_window_frame(ctx: &egui::Context, theme: &Theme) -> egui::CornerRadius{
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

    let corner_radius = if is_maximized {
        egui::CornerRadius::ZERO
    } else {
        egui::CornerRadius::same(12)
    };

    windows::draw_resize_handles(ctx, is_maximized);

    let screen_rect = ctx.content_rect();
    ctx.layer_painter(egui::LayerId::background()).rect_filled(
        screen_rect,
        corner_radius,
        theme.bg,
    );

    corner_radius
}

// Function to draw resize handles on window edges and corners
pub(crate) fn draw_resize_handles(ctx: &egui::Context, is_maximized: bool) {
    // Don't show resize handles when maximized
    if is_maximized {
        return;
    }

    let screen_rect = ctx.content_rect();
    let handle_size = 8.0; // Size of the resize handle area
    let corner_size = 16.0; // Larger size for corners

    // Helper function to create a resize handle using Area
    let create_handle = |id: &str, pos: egui::Pos2, size: egui::Vec2, direction: egui::viewport::ResizeDirection, cursor: egui::CursorIcon| {
        egui::Area::new(egui::Id::new(id))
            .fixed_pos(pos)
            .interactable(true)
            .show(ctx, |ui| {
                let (_rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
                
                if response.hovered() {
                    ui.ctx().set_cursor_icon(cursor);
                }

                if response.drag_started() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::BeginResize(direction));
                }
            });
    };

    // Corner handles (priority areas)
    create_handle(
        "resize_nw",
        screen_rect.left_top(),
        egui::vec2(corner_size, corner_size),
        egui::viewport::ResizeDirection::NorthWest,
        egui::CursorIcon::ResizeNwSe,
    );

    create_handle(
        "resize_ne",
        egui::pos2(screen_rect.right() - corner_size, screen_rect.top()),
        egui::vec2(corner_size, corner_size),
        egui::viewport::ResizeDirection::NorthEast,
        egui::CursorIcon::ResizeNeSw,
    );

    create_handle(
        "resize_sw",
        egui::pos2(screen_rect.left(), screen_rect.bottom() - corner_size),
        egui::vec2(corner_size, corner_size),
        egui::viewport::ResizeDirection::SouthWest,
        egui::CursorIcon::ResizeNeSw,
    );

    create_handle(
        "resize_se",
        egui::pos2(screen_rect.right() - corner_size, screen_rect.bottom() - corner_size),
        egui::vec2(corner_size, corner_size),
        egui::viewport::ResizeDirection::SouthEast,
        egui::CursorIcon::ResizeNwSe,
    );

    // Edge handles (avoid corners)
    // Top edge
    create_handle(
        "resize_n",
        egui::pos2(screen_rect.left() + corner_size, screen_rect.top()),
        egui::vec2(screen_rect.width() - 2.0 * corner_size, handle_size),
        egui::viewport::ResizeDirection::North,
        egui::CursorIcon::ResizeVertical,
    );

    // Bottom edge
    create_handle(
        "resize_s",
        egui::pos2(screen_rect.left() + corner_size, screen_rect.bottom() - handle_size),
        egui::vec2(screen_rect.width() - 2.0 * corner_size, handle_size),
        egui::viewport::ResizeDirection::South,
        egui::CursorIcon::ResizeVertical,
    );

    // Left edge
    create_handle(
        "resize_w",
        egui::pos2(screen_rect.left(), screen_rect.top() + corner_size),
        egui::vec2(handle_size, screen_rect.height() - 2.0 * corner_size),
        egui::viewport::ResizeDirection::West,
        egui::CursorIcon::ResizeHorizontal,
    );

    // Right edge
    create_handle(
        "resize_e",
        egui::pos2(screen_rect.right() - handle_size, screen_rect.top() + corner_size),
        egui::vec2(handle_size, screen_rect.height() - 2.0 * corner_size),
        egui::viewport::ResizeDirection::East,
        egui::CursorIcon::ResizeHorizontal,
    );
}
