use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    vspace,
    property_dropdown,
    property_section,
    property_str,
    property_int
};
use bevy_egui::egui::{self};
use mn_core::MonoTab;
use mn_core::enums::AnchorStrategy;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {

    // Temporary
    let mut string: String = "".to_string();
    let mut integer: f64 = 0.0;
    let mut anchor_strategy = AnchorStrategy::RayCast;
    // Temporary

    property_section(
        ui,
        theme,
        "Transform",
        format!("transform_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Host Level", &mut string);         
            property_int(ui, w, "Offset", &mut integer, 0.0..=10.0);
            vspace(ui);
            property_int(ui, w, "Local Position X", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Y", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Z", &mut integer, 0.0..=10.0);
            vspace(ui);
            property_int(ui, w, "Rotation X", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Y", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Z", &mut integer, 0.0..=10.0);
            vspace(ui);
        },
    );

    property_section(
        ui,
        theme,
        "Global Coordinates",
        format!("global_coord{}", tab.id),
        |ui, w| {
            property_int(ui, w, "Global Position X", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Y", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Z", &mut integer, 0.0..=10.0);
            vspace(ui);
            property_int(ui, w, "Latitude", &mut integer, 0.0..=10.0);
            property_int(ui, w, "Longitude", &mut integer, 0.0..=10.0);
        },
    );
    let mut string: String = "".to_string();
    property_section(
        ui,
        theme,
        "Constraint",
        format!("constraint_{}", tab.id),
        |ui, w| {
            property_dropdown(ui, w, tab, "Anchor Strategy", &mut anchor_strategy);
        },
    );

    property_section(
        ui,
        theme, 
        "Bounding Box",
        format!("bounding_box{}", tab.id),
        |ui, w| {
            property_int(ui, w, "Dimension L", &mut integer, 0.0..=10.0);
            property_int(ui, w, "W", &mut integer, 0.0..=10.0);
            property_int(ui, w, "ZH", &mut integer, 0.0..=10.0);
            vspace(ui);
            property_str(ui, w, "Clash Status", &mut string);         
        },
    );

}
