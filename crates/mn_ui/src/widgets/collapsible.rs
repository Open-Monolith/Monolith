use crate::theme::ThemeResource;
use bevy_egui::egui::{self, CollapsingHeader, Ui};

pub fn property_section<F>(
    ui: &mut Ui,
    theme: &ThemeResource,
    title: &str,
    id_salt: impl std::hash::Hash,
    content_builder: F)
where
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
                        .spacing([0.0, 2.0])
                        .show(ui, |ui| {
                            content_builder(ui, widths);
                        });
                });
        });
}

pub fn property_row(
    ui: &mut egui::Ui,
    w: egui::Vec2,
    label: &str,
    widget: impl egui::Widget, // <--- Accepts the Widget directly (DragValue, Button, etc.)
) -> egui::Response {
    let row_h = ui.spacing().interact_size.y;

    // 1. Draw Label (Column 1)


    ui.allocate_ui_with_layout(
        egui::Vec2::new(w[0] - 10.0, row_h),                 // subtract a little padding if you want
        egui::Layout::left_to_right(egui::Align::Center),           // align content to the right
        |ui: &mut Ui| {
            ui.add_sized(
                [w[0]-10., row_h],
                egui::Label::new(label)
            );
        },
    );

    // 2. Draw Widget (Column 2) inside add_sized
    // This forces the DragValue/Button/Checkbox to fill the calculated space.
    let response = ui.add_sized(
        [w[1]-10., row_h],
        widget
    );

    ui.end_row();
    
    response
}