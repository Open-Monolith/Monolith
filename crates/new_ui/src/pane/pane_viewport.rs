use std::collections::HashMap;
use bevy_egui::egui;
use new_core::Pane;

pub fn show(
    ui: &mut egui::Ui,
    pane: &mut Pane
) {
    let rect = ui.max_rect();

    ui.painter().rect_stroke(
        rect,
        0.0,
        egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
        egui::StrokeKind::Inside,
    );

    // let rect = ui.max_rect();
    // visible_viewports.insert(pane.id, rect);
    let _ = ui.allocate_rect(rect, egui::Sense::hover());
}