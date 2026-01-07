use bevy_egui::egui;
use mn_core::MonoTab;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab) {
    ui.label(format!("Tab title is {}", tab.title));
}