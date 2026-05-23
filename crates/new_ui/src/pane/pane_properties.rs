use bevy_egui::egui;
use crate::utils::paint_opaque_pane_background;

pub fn show(
    ui: &mut egui::Ui
) {
    paint_opaque_pane_background(ui);

    ui.heading("Properties");
    ui.horizontal(|ui| {
        ui.label("Filter:");
    });
}