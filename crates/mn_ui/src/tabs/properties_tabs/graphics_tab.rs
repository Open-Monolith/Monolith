use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    vspace,
    property_dropdown,
    property_section,
    property_str,
    property_slider,
    property_checkbox,
};
use bevy_egui::egui::{self};
use mn_core::MonoTab;
use mn_core::enums::{
    LOD, CutBehavior, RenderMode
};

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {
    
    // Temporary
    let mut string: String = "".to_string();
    let mut integer: f64 = 0.0;
    let mut boolean: bool = true;

    let mut lod_maturity = LOD::Lod100;
    let mut cut_behavior = CutBehavior::ShowTop;
    let mut render_mode = RenderMode::Automatic;
    // Temporary

    property_section(
        ui,
        theme,
        "Global Asset Settings",
        format!("global_asset_settings_{}", tab.id),
        |ui, w| {
   
            property_dropdown(ui, w, tab, "BIM LOD", &mut lod_maturity);
            property_str(ui, w, "Plan Symbol", &mut string);
        },
    );

    property_section(
        ui,
        theme, 
        "Local Override",
        format!("bounding_box_{}", tab.id),
        |ui, w: egui::Vec2| {
            property_dropdown(ui, w, tab, "BIM LOD", &mut lod_maturity);
            vspace(ui);
            property_checkbox(ui, w, "Visible", &mut boolean);
            property_checkbox(ui, w, "Half-Tone", &mut boolean);
            property_slider(ui, w, "Transparency", &mut integer, 0.0..=10.0);
            vspace(ui);
            property_dropdown(ui, w, tab, "Cut Behavior", &mut cut_behavior);
        },
    );

    property_section(
        ui,
        theme, 
        "Geometric Fidelity",
        format!("geometric_fidelity_{}", tab.id),
        |ui, w: egui::Vec2| {
            property_dropdown(ui, w, tab, "Render Mode", &mut render_mode);
        },
    );

}