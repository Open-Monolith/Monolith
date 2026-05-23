use bevy_egui::egui;

pub fn paint_opaque_pane_background(ui: &egui::Ui) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);
}