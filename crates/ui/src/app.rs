use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::tabs::{MyTab, MyTabViewer, TabKind};
use crate::utils::{hex_to_color};
use crate::widgets::{menubar, windows};



// MyApp
pub(crate) struct MyApp {
    pub tree: DockState<MyTab>,
    pub search_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let tab1 = MyTab {
            kind: TabKind::SceneView,
            title: "SceneView".to_string(),
            id: 1,
        };

        let tab2 = MyTab {
            kind: TabKind::Explorer,
            title: "Explorer".to_string(),
            id: 3,
        };

        let tab3 = MyTab {
            kind: TabKind::Properties,
            title: "Properties".to_string(),
            id: 2,
        };

        let mut tree = DockState::new(vec![tab1]);

        let [_, b] = tree
            .main_surface_mut()
            .split_right(NodeIndex::root(), 0.7, vec![tab2]);

        let [_, _] = tree.main_surface_mut().split_below(b, 0.5, vec![tab3]);

        Self { tree, search_text: String::new() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        let bg_color = hex_to_color("#181818");
        let panel_color = hex_to_color("#282828");

        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        let corner_radius = if is_maximized {
            egui::CornerRadius::ZERO
        } else {
            egui::CornerRadius::same(12)
        };

        let screen_rect = ctx.content_rect();
        ctx.layer_painter(egui::LayerId::background()).rect_filled(
            screen_rect,
            corner_radius,
            bg_color
        );

        egui::TopBottomPanel::top("main_menu_bar")
            .show_separator_line(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(15, 8),
                fill: bg_color,
                stroke: egui::Stroke::NONE,
                corner_radius,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    menubar::menu_bar(self, ctx, ui)
                });
            });

        // Draw resize handles around the window edges
        windows::draw_resize_handles(ctx, is_maximized);

        let mut style = Style::from_egui(ctx.style().as_ref());
        style.separator.width = 5.0;
        style.separator.extra_interact_width = 4.0;
        style.separator.color_idle = bg_color;
        style.separator.color_hovered = bg_color;
        style.separator.color_dragged = bg_color;
        style.tab.tab_body.stroke = egui::Stroke::NONE;

        style.tab.active.outline_color = egui::Color32::TRANSPARENT;
        style.tab.inactive.outline_color = egui::Color32::TRANSPARENT;
        style.tab.hovered.outline_color = egui::Color32::TRANSPARENT;
        style.tab.focused.outline_color = egui::Color32::TRANSPARENT;
        style.tab.active_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;
        style.tab.inactive_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;
        style.tab.focused_with_kb_focus.outline_color = egui::Color32::TRANSPARENT;

        style.tab.active.bg_fill = panel_color;
        style.tab.inactive.bg_fill = panel_color;
        style.tab.hovered.bg_fill = panel_color;
        style.tab.focused.bg_fill = panel_color;

        style.tab_bar.bg_fill = bg_color;
        style.tab.tab_body.bg_fill = panel_color;

        DockArea::new(&mut self.tree)
            .style(style)
            .show_leaf_collapse_buttons(false)
            .show_close_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut MyTabViewer {});
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

