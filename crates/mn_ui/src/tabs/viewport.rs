use bevy_egui::egui;
use mn_core::MonoTab;

use crate::viewer::MyTabViewer;

pub fn show(ui: &mut egui::Ui, viewer: &mut MyTabViewer, tab: &mut MonoTab) {
    let rect = ui.available_rect_before_wrap();

    if rect.width() < 0.0 && rect.height() < 0.0 {
        return;
    }

    // Reserve space so the layout doesn't collapse
    ui.allocate_rect(rect, egui::Sense::hover());

    viewer.viewports.insert(tab.id, rect);

    // Draw a placeholder background (optional, but good for debugging)
    // We use transparent so Bevy shows through, or dark grey to hide glitches
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::ZERO,
        egui::Color32::from_black_alpha(0),
    );
}
