use crate::theme::ThemeResource;
use crate::widgets::property_collapsible::{
    vspace,
    property_dropdown,
    property_section,
    property_str
};
use bevy_egui::egui::{self};
use mn_core::MonoTab;
use mn_core::enums::PhasingTemp;

pub fn show(ui: &mut egui::Ui, tab: &mut MonoTab, theme: &ThemeResource) {

    // Temporary
    let mut string: String = "".to_string();
    let mut temp_phasing: PhasingTemp = PhasingTemp::Demolition;
    // Temporary

    property_section(
        ui,
        theme,
        "Identity Data",
        format!("identity_data_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Asset Mark", &mut string);
            property_str(ui, w, "Comments", &mut string);
            vspace(ui);
            property_dropdown(ui, w, tab, "Phasing", &mut temp_phasing);
        },
    );

    property_section(
        ui,
        theme,
        "Asset Definition",
        format!("asset_definition_{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Manufacturer", &mut string);
            property_str(ui, w, "Model", &mut string);
            property_str(ui, w, "URL", &mut string);
            property_str(ui, w, "Cost", &mut string);
            property_str(ui, w, "OmniClass", &mut string);
        },
    );

    property_section(
        ui,
        theme,
        "Custom Traits",
        format!("custom_traits{}", tab.id),
        |ui, w| {
            property_str(ui, w, "Custom", &mut string);
        },
    );
}
