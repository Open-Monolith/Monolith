use bevy::ecs::{resource::Resource, system::Commands};
use bevy_egui::{ 
    egui::{self} 
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Dark,
    Light
}


#[derive(Clone, Debug)]
pub struct Palette {
    // general
    pub bg: egui::Color32,
    pub panel: egui::Color32,
    pub property: egui::Color32,
    pub values: egui::Color32,

    // Widgets
    pub widget_bg: egui::Color32,
    pub widget_weak_bg: egui::Color32,
    pub widget_stroke: egui::Stroke,
    
    // Selection
    pub selection_bg: egui::Color32,
    pub selection_stroke: egui::Stroke,

    // Font
    pub text_color: Option<egui::Color32>,

    // Misc
    pub accent: egui::Color32,

    pub button: egui::Color32,

}

#[derive(Clone, Debug)]
pub struct Values {
    pub label_font_size: f32,
    pub numeric_font_size: f32,
}

#[derive(Resource, Debug)]
pub struct ThemeResource {
    pub mode: Mode,
    pub dark: Palette,
    pub light: Palette,
    pub values: Values
}


impl Default for ThemeResource {
    fn default() -> Self {

        // default dark palette
        let dark = Palette {
            bg: hex_to_color("#1D1D1D"),
            panel: hex_to_color("#282828"),
            property: hex_to_color("#3D3D3D"),
            values: hex_to_color("#545454"),
            widget_bg: hex_to_color("#2A2A2A"),
            widget_weak_bg: hex_to_color("#222222"),
            widget_stroke: egui::Stroke::NONE,
            selection_bg: hex_to_color("#3A6EA5"),
            selection_stroke: egui::Stroke::NONE,
            text_color: Some(egui::Color32::from_gray(242)),
            accent: hex_to_color("#3A6EA5"),
            
            button: hex_to_color("#E0E0E0"),
        };

        // default light palette
        let light = Palette {
            bg: hex_to_color("#E0E0E0"),
            panel: hex_to_color("#F0F0F0"),
            property: hex_to_color("#d1d1d1"),
            values: hex_to_color("#bcbcbc"),
            widget_bg: hex_to_color("#EBEBEB"),
            widget_weak_bg: hex_to_color("#F5F5F5"),
            widget_stroke: egui::Stroke::new(1.0, hex_to_color("#D0D0D0")),
            selection_bg: hex_to_color("#3A6EA5"),
            selection_stroke: egui::Stroke::NONE,
            text_color: Some(egui::Color32::from_gray(40)),
            accent: hex_to_color("#3A6EA5"),

            button: hex_to_color("#282828"),
        };

        let values = Values {
            label_font_size: 12.0,
            numeric_font_size: 12.0,
        };

        Self {
            mode: Mode::Dark,
            dark: dark,
            light: light,
            values: values
        }
    }
}



impl ThemeResource {
    pub fn current(&self) -> &Palette {
        match self.mode {
            Mode::Dark => &self.dark,
            Mode::Light => &self.light
        }
    }

    pub fn toggle_mode(&mut self, ctx: &egui::Context) {
        self.mode = match self.mode {
            Mode::Dark => Mode::Light,
            Mode::Light => Mode::Dark,
        };
        self.apply_to_ctx(ctx);
    } 

    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        let mut style: egui::Style = ctx.style().as_ref().clone();
        let theme = self.current();
        let values = &self.values;

        style.visuals.extreme_bg_color = theme.values; // <--- ADD THIS

        // Widgets
        style.visuals.widgets.inactive.bg_fill = theme.values;
        style.visuals.widgets.inactive.weak_bg_fill = theme.values;
        style.visuals.widgets.inactive.bg_stroke = theme.widget_stroke;

        // Selection
        style.visuals.selection.bg_fill = theme.selection_bg;
        style.visuals.selection.stroke = theme.selection_stroke;

        // other visual fields you might want to control:
        style.visuals.window_fill = theme.panel;
        // text color (egui::Visuals stores text_color Option)
        style.visuals.override_text_color = theme.text_color; 

        
        // Font Size
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(values.label_font_size),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(values.label_font_size),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::proportional((values.label_font_size * 0.85).max(8.0)),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::monospace(values.numeric_font_size),
        );

        ctx.set_style(style);
        ctx.request_repaint();
    }

    pub fn to_dock_style(&self, ctx: &egui::Context) -> egui_dock::Style {

        let  mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
        let theme = self.current();

        // Separator
        style.separator.width = 3.0;
        style.separator.extra_interact_width = 4.0;
        style.separator.color_idle = theme.bg;
        style.separator.color_hovered = theme.bg;
        style.separator.color_dragged = theme.bg;
        style.tab_bar.bg_fill = theme.panel;

        // Tab
        style.tab_bar.corner_radius = egui::CornerRadius::ZERO;
        // style.tab.tab_body.stroke = egui::Stroke::new(1.0, hex_to_color("#414141"));
        style.tab.tab_body.stroke = egui::Stroke::NONE;
        style.tab.tab_body.corner_radius = egui::CornerRadius::ZERO;
        style.tab.tab_body.inner_margin = egui::Margin::same(0); 

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

}

 
pub fn configure_theme_startup(mut contexts: bevy_egui::EguiContexts, mut commands: Commands) {
    if let Ok(ctx) = contexts.ctx_mut() {

        let theme_res = ThemeResource::default();
        
        theme_res.apply_to_ctx(&ctx);

        setup_fonts(&ctx);

    }
}



pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts: egui::FontDefinitions = egui::FontDefinitions::default();
    const FONT_NAME: &str = "Inter";
    
    let mut font_data = egui::FontData::from_static(include_bytes!(
        "../../mn_app/assets/ui/fonts/Inter-Light-4.1.ttf"
    ));

    font_data.tweak = egui::FontTweak {
        scale: 1.0,
        y_offset_factor: 0.0,
        y_offset: 0.0,
    };
    
    fonts.font_data.insert(FONT_NAME.to_owned(), std::sync::Arc::new(font_data));

    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, FONT_NAME.to_owned());

    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push(FONT_NAME.to_owned());

    ctx.set_fonts(fonts);
}



/// helper hex -> egui::Color32
pub fn hex_to_color(hex: &str) -> egui::Color32 {
    let s = hex.trim_start_matches('#');
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            egui::Color32::from_rgb(r, g, b)
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
            egui::Color32::from_rgba_unmultiplied(r, g, b, a)
        }
        _ => egui::Color32::BLACK,
    }
}

