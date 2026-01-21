use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{property_row, property_section, vspace};
use bevy_egui::egui::{self};
use mn_core::MonoTab;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {
    property_section(
        ui,
        theme,
        "Transform",
        format!("transform_{}", tab.id),
        |ui, w| {
            property_row(ui, w, "Host Level", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Offset", egui::DragValue::new(&mut 1).speed(1.0));
            vspace(ui);
            property_row(ui, w, "Local Position X", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Y", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Z", egui::DragValue::new(&mut 1).speed(1.0));
            vspace(ui);
            property_row(ui, w, "Rotation X", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Y", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Z", egui::DragValue::new(&mut 1).speed(1.0));
            vspace(ui);
        },
    );

    property_section(
        ui,
        theme,
        "Global Coordinates",
        format!("global_coord{}", tab.id),
        |ui, w| {
            property_row(ui, w, "Global Position X", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Y", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Z", egui::DragValue::new(&mut 1).speed(1.0));
            vspace(ui);
            property_row(ui, w, "GIS Latitude", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "Longitude", egui::DragValue::new(&mut 1).speed(1.0));
        },
    );
    
    property_section(
        ui,
        theme,
        "Constraint",
        format!("constraint_{}", tab.id),
        |ui, w| {
            property_row(ui, w, "Anchor Strategy", egui::DragValue::new(&mut 1).speed(1.0));
        },
    );

    property_section(
        ui,
        theme,
        "Bounding Box",
        format!("bounding_box{}", tab.id),
        |ui, w| {
            property_row(ui, w, "Dimension L", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "W", egui::DragValue::new(&mut 1).speed(1.0));
            property_row(ui, w, "H", egui::DragValue::new(&mut 1).speed(1.0));
            vspace(ui);
            property_row(ui, w, "Clash Status", egui::DragValue::new(&mut 1).speed(1.0));
        },
    );

}
