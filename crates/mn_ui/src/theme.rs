use bevy_egui::{
    EguiContexts, egui::{self, FontData, FontFamily}
};
use egui_dock::Style;

pub struct Theme {
    pub bg: egui::Color32,
    pub panel: egui::Color32,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            bg: hex_to_color("#181818"),
            panel: hex_to_color("#282828"),
        }
    }

    pub fn light() -> Self {
        Self {
            bg: hex_to_color("#FFFFFF"),
            panel: hex_to_color("#E0E0E0"),
        }
    }

    pub fn from_ctx(ctx: &egui::Context) -> Self {
        if ctx.style().visuals.dark_mode {
            Self::dark()
        } else {
            Self::light()
        }
    }
}

// Startup system call
pub fn configure_theme(mut contexts: EguiContexts) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    ctx.set_visuals(egui::Visuals::dark());
    setup_fonts(ctx);
}

pub fn get_dock_style(ctx: &egui::Context, theme: &Theme) -> Style {
    let mut style = Style::from_egui(ctx.style().as_ref());

    // Separator
    style.separator.width = 5.0;
    style.separator.extra_interact_width = 4.0;
    style.separator.color_idle = theme.bg;
    style.separator.color_hovered = theme.bg;
    style.separator.color_dragged = theme.bg;
    style.tab_bar.bg_fill = theme.panel;

    // Tab
    style.tab.tab_body.stroke = egui::Stroke::NONE;
    style.tab_bar.corner_radius = egui::CornerRadius::ZERO;
    style.tab.tab_body.corner_radius = egui::CornerRadius::ZERO;
    // Tab Outline
    style.tab.active.outline_color = egui::Color32::TRANSPARENT;
    style.tab.inactive.outline_color = egui::Color32::TRANSPARENT;
    style.tab.hovered.outline_color = egui::Color32::TRANSPARENT;
    style.tab.focused.outline_color = egui::Color32::TRANSPARENT;
    style.tab.active_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;
    style.tab.inactive_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;
    style.tab.focused_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;
    style.tab_bar.hline_color = egui::Color32::TRANSPARENT;

    // Tab BG
    style.tab.active.bg_fill = theme.panel;
    style.tab.inactive.bg_fill = theme.panel;
    style.tab.hovered.bg_fill = theme.panel;
    style.tab.focused.bg_fill = theme.panel;

    style.tab_bar.bg_fill = theme.bg;
    style.tab.tab_body.bg_fill = theme.panel;

    style
}

pub fn hex_to_color(hex: &str) -> egui::Color32 {
    let hex: &str = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

pub fn setup_fonts(ctx: &mut bevy_egui::egui::Context) {
    let mut fonts: egui::FontDefinitions = egui::FontDefinitions::default();
    const FONT_NAME: &str = "Inter";

    fonts.font_data.insert(
        FONT_NAME.to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../../mn_app/assets/ui/fonts/Inter-Light-4.1.ttf"
        ))),
    );

    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
    .insert(0, FONT_NAME.to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push(FONT_NAME.to_owned());

    ctx.set_fonts(fonts);
}

