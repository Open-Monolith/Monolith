use eframe::egui;
use crate::docktabs::{MyTab};

pub fn show(ui: &mut egui::Ui, tab: &mut MyTab) {
    ui.label(format!("Tab title is {}", tab.title));
}