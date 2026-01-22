use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    property_section,
    property_str,
    property_int
};
use bevy_egui::egui::{self};
use mn_core::MonoTab;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {
    // Temporary
    let mut string: String = "".to_string();
    let mut integer: f64 = 0.0;
    // Temporary

    property_section(
        ui,
        theme,
        "Connections",
        format!("connections_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Port 1 (Inlet)", &mut string);
            property_str(ui, w, "Port 2 (Outlet)", &mut string);
            property_str(ui, w, "Electrical System", &mut string);
        },
    );

    property_section(
        ui,
        theme, 
        "Mechanical",
        format!("mechanical_{}", tab.id),
        |ui, w| {
            property_int(ui, w, "System", &mut integer, 0.0..=3.0);
            property_int(ui, w, "Flow Rate", &mut integer, 0.0..=3.0);
            property_int(ui, w, "Pressure Drop", &mut integer, 0.0..=3.0);
        },
    );

}
