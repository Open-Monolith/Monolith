use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    property_section,
    property_str,
};
use bevy_egui::egui::{self};
use mn_core::MonoTab;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {
    
    // Temporary
    let mut string: String = "".to_string();

    // Temporary

    property_section(
        ui,
        theme,
        "Status",
        format!("status_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Creation Commit", &mut string);
            property_str(ui, w, "Last Modified", &mut string);
            property_str(ui, w, "Branch", &mut string);
        },
    );

    property_section(
        ui,
        theme,
        "History",
        format!("history_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "[1 day ago 10:45 PM]", &mut string);
            property_str(ui, w, "[2 day ago 4:00 PM]", &mut string);
            property_str(ui, w, "[10 Dec 2025]", &mut string);
        },
    );

    property_section(
        ui,
        theme,
        "Threads",
        format!("threads_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Active Threads", &mut string);
            property_str(ui, w, "Quick Actions", &mut string);
        },
    );
}