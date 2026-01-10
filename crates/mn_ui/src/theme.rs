use bevy::ecs::{resource::Resource, system::Commands};
use bevy_egui::{ 
    egui::{self} 
};

#[derive(Clone, Debug)]
pub enum Mode {
    Dark,
    Light
}


#[derive(Clone, Debug)]
pub struct Palette {
    // general
    pub bg: egui::Color32,
    pub panel: egui::Color32,

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
}

#[derive(Resource, Debug)]
pub struct ThemeResource {
    pub mode: Mode,
    pub dark: Palette,
    pub light: Palette
}


impl Default for ThemeResource {
    fn default() -> Self {

        // default dark palette
        let dark = Palette {
            bg: hex_to_color("#181818"),
            panel: hex_to_color("#282828"),
            widget_bg: hex_to_color("#2A2A2A"),
            widget_weak_bg: hex_to_color("#222222"),
            widget_stroke: egui::Stroke::NONE,
            selection_bg: hex_to_color("#3A6EA5"),
            selection_stroke: egui::Stroke::NONE,
            text_color: Some(egui::Color32::from_gray(242)),
            accent: hex_to_color("#3A6EA5"),
        };

        // default light palette
        let light = Palette {
            bg: hex_to_color("#FFFFFF"),
            panel: hex_to_color("#E6E6E6"),
            widget_bg: hex_to_color("#F5F5F5"),
            widget_weak_bg: hex_to_color("#FAFAFA"),
            widget_stroke: egui::Stroke::new(1.0, hex_to_color("#CCCCCC")),
            selection_bg: hex_to_color("#2A6EA6"),
            selection_stroke: egui::Stroke::new(2.0, egui::Color32::WHITE),
            text_color: Some(egui::Color32::from_gray(20)),
            accent: hex_to_color("#2A6EA6"),
        };

        Self {
            mode: Mode::Dark,
            dark: dark,
            light: light
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

        // Widgets
        style.visuals.widgets.inactive.bg_fill = theme.widget_bg;
        style.visuals.widgets.inactive.weak_bg_fill = theme.widget_weak_bg;
        style.visuals.widgets.inactive.bg_stroke = theme.widget_stroke;
        
        // Selection
        style.visuals.selection.bg_fill = theme.selection_bg;
        style.visuals.selection.stroke = theme.selection_stroke;

        // other visual fields you might want to control:
        style.visuals.window_fill = theme.panel;
        // text color (egui::Visuals stores text_color Option)
        style.visuals.override_text_color = theme.text_color; 

        ctx.set_style(style);
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

}

 
// Startup system call
pub fn configure_theme_startup(mut contexts: bevy_egui::EguiContexts, mut commands: Commands) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // build default resource and apply to ctx

        let theme_res = ThemeResource::default();

        theme_res.apply_to_ctx(&ctx);

        setup_fonts(&ctx);

        commands.insert_resource(theme_res); // init at lib cause it would crash without it. this line simply overwrites
    }
}



pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts: egui::FontDefinitions = egui::FontDefinitions::default();
    const FONT_NAME: &str = "Inter";

    fonts.font_data.insert(
        FONT_NAME.to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../mn_app/assets/ui/fonts/Inter-Light-4.1.ttf"
        ))),
    );

    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
    .insert(0, FONT_NAME.to_owned());

    // Put my font as last fallback for monospace:
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

