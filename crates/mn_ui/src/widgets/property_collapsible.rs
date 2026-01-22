use std::ops::RangeInclusive;

use crate::theme::ThemeResource;
use bevy_egui::egui::{self, CollapsingHeader, Ui};
use mn_core::MonoTab;
use strum::IntoEnumIterator;

pub fn property_section<F>(
    ui: &mut Ui,
    theme: &ThemeResource,
    title: &str,
    id_salt: impl std::hash::Hash,
    content_builder: F,
) where
    F: FnOnce(&mut Ui, egui::Vec2),
{
    let palette = theme.current();

    egui::Frame::NONE
        .fill(palette.property)
        .inner_margin(egui::Margin::same(6))
        .outer_margin(egui::Margin::ZERO)
        .corner_radius(4)
        .show(ui, |ui| {
            // 1. Force Frame to take full width of parent
            let available_w = ui.available_width();
            ui.set_min_width(available_w);

            // 2. Calculate Column Widths
            // We subtract a safety buffer (e.g. 10.0 or 20.0) to account for scrollbars/padding
            // so we don't overflow the container.
            let safety_margin = 10.0;
            let usable_w = (available_w - safety_margin).max(1.0);

            let col1_percent = 0.45;
            let col2_percent = 0.55;

            let col1_w = (usable_w * col1_percent).floor();
            let col2_w = (usable_w * col2_percent).floor();
            let widths = egui::vec2(col1_w, col2_w);

            CollapsingHeader::new(title)
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new(id_salt)
                        .num_columns(2)
                        .min_col_width(0.0)
                        .min_row_height(1.0)
                        .spacing([3.0, 2.0])
                        .show(ui, |ui| {
                            content_builder(ui, widths);
                        });
                });
        });
}

pub fn property_row<F>(ui: &mut egui::Ui, w: egui::Vec2, label: &str, contents: F)
where
    F: FnOnce(&mut egui::Ui),
{
    const R_PADDING: f32 = 10.0;
    let row_h = ui.spacing().interact_size.y;

    let (left_rect, _) =
        ui.allocate_exact_size(egui::vec2(w[0] - R_PADDING, row_h), egui::Sense::hover());

    let font_id = egui::TextStyle::Body.resolve(ui.style());
    let text_color = ui.style().visuals.text_color();

    ui.painter().text(
        left_rect.right_center() - egui::vec2(2.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        label,
        font_id,
        text_color,
    );

    let right_rect = egui::vec2((w[1] - R_PADDING).max(0.0), row_h);
    ui.allocate_ui(right_rect, |child_ui| {
        contents(child_ui);
    });

    ui.end_row();
}

pub fn vspace(ui: &mut egui::Ui) {
    const HEIGHT: f32 = 2.0;

    ui.allocate_exact_size(egui::vec2(1.0, HEIGHT), egui::Sense::hover());
    ui.allocate_exact_size(egui::vec2(1.0, HEIGHT), egui::Sense::hover());
    ui.end_row();
}

pub fn property_str(ui: &mut egui::Ui, w: egui::Vec2, label: &str, value: &mut String) {
    property_row(ui, w, label, |ui| {
        ui.add(egui::TextEdit::singleline(value));
    });
}

pub fn property_int<T>(
    ui: &mut egui::Ui,
    w: egui::Vec2,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
) where
    T: egui::emath::Numeric,
{
    property_row(ui, w, label, |ui| {
        ui.add_sized(
            egui::vec2(w[1] - 10.0, ui.spacing().interact_size.y),
            egui::DragValue::new(value)
                .range(range)
                .clamp_existing_to_range(true)
                .speed(1.0),
        );
    });
}

pub fn property_slider<T>(
    ui: &mut egui::Ui,
    w: egui::Vec2,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
    // step: &T
) where
    T: egui::emath::Numeric,
{
    property_row(ui, w, label, |ui| {
        ui.add_sized(
            egui::vec2(w[1] - 50.0, ui.spacing().interact_size.y),
            egui::Slider::new(value, range)
                .handle_shape(egui::style::HandleShape::Circle)
                .trailing_fill(true)
        );
    });
}

pub fn property_dropdown<T>(
    ui: &mut egui::Ui,
    w: egui::Vec2,
    tab: &mut MonoTab,
    label: &str,
    value: &mut T,
) where
    T: IntoEnumIterator + Copy + PartialEq + std::fmt::Debug + std::fmt::Display,
{
    property_row(ui, w, label, |ui| {
        egui::ComboBox::from_id_salt(format!("Combobox_{}_{}", label.replace(" ", ""), tab.id))
            .selected_text(value.to_string())
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for v in T::iter() {
                    ui.selectable_value(value, v, v.to_string());
                }
            });
    });
}

pub fn property_checkbox(
    ui: &mut egui::Ui,
    w: egui::Vec2,
    label: &str,
    value: &mut bool,
) {
    property_row(ui, w, label, |ui| {
        ui.checkbox(value, "");
    });
}



// pub fn combined_property_row<F>(
//     ui: &mut egui::Ui,
//     w: egui::Vec2,
//     label: &str,
//     contents: F
// ) 
// where
//     F: FnOnce(&mut egui::Ui)
// {
//     const R_PADDING: f32 = 10.0;
//     let row_h = ui.spacing().interact_size.y;

//     let (left_rect, _) =
//         ui.allocate_exact_size(egui::vec2(w[0] - R_PADDING, row_h), egui::Sense::hover());

//     let font_id = egui::TextStyle::Body.resolve(ui.style());
//     let text_color = ui.style().visuals.text_color();

//     ui.painter().text(
//         left_rect.right_center() - egui::vec2(2.0, 0.0),
//         egui::Align2::RIGHT_CENTER,
//         label,
//         font_id,
//         text_color,
//     );

//     let right_rect = egui::vec2((w[1] - R_PADDING).max(0.0), row_h);
//     ui.allocate_ui(right_rect, |ui| {
//         ui.vertical(|ui| {
//             contents(ui);
//         })
//     });

//     ui.end_row();
// }


// pub fn property_checkbox(
//     ui: &mut egui::Ui,
//     value: &mut bool,
//     label: &str
// ) {
//     ui.checkbox(value, label);
// }
