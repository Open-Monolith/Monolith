use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    property_section,
    property_str
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
        "Modifiers",
        format!("modifier_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Array", &mut string);
            property_str(ui, w, "Boolean", &mut string);
            property_str(ui, w, "Mirror", &mut string);
            property_str(ui, w, "Duplicate", &mut string);
        },
    );

    property_section(
        ui,
        theme, 
        "Script",
        format!("bounding_box{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Lua Script", &mut string);
            property_str(ui, w, "Variables", &mut string);
        },
    );

}
