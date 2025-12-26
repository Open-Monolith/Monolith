use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex};

use crate::docktabs::{MyTab, MyTabViewer, TabKind};
use crate::theme::{self, Theme};
use crate::widgets::windows::draw_window_frame;
use crate::widgets::{menubar};

// MyApp
pub(crate) struct MyApp {
    pub tree: DockState<MyTab>,
    pub search_text: String,
    pub tab_id_counter: u32,
}

impl MyApp {
    pub fn create_tab(&mut self, kind: TabKind) -> MyTab {
        self.tab_id_counter += 1;
        MyTab {
            kind: kind.clone(),
            title: format!("{:?}", kind),
            id: self.tab_id_counter,
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        // Create default tabs
        let mut local_id_counter = 1;

        let mut next_tab = |kind: TabKind| -> MyTab {
            let tab = MyTab {
                kind: kind.clone(),
                title: format!("{:?}", kind),
                id: local_id_counter,
            };
            local_id_counter += 1;
            tab
        };

        let tab1 = next_tab(TabKind::SceneView);
        let tab2 = next_tab(TabKind::Explorer);
        let tab3 = next_tab(TabKind::Properties);
        let mut tree = DockState::new(vec![tab1]);

        // Apply Dock layout
        let [_, b] = tree
            .main_surface_mut()
            .split_right(NodeIndex::root(), 0.7, vec![tab2]);

        let [_, _] = tree.main_surface_mut().split_below(b, 0.5, vec![tab3]);

        // Return Self
        Self {
            tree,
            search_text: String::new(),
            tab_id_counter: local_id_counter
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        
        let theme: Theme = Theme::from_ctx(ctx);

        // Draw Window Frame
        let corner_radius: egui::CornerRadius = draw_window_frame(ctx, &theme);

        // Top Menu Bar
        egui::TopBottomPanel::top("main_menu_bar")
            .show_separator_line(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(15, 8),
                fill: theme.bg,
                stroke: egui::Stroke::NONE,
                corner_radius,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| menubar::menu_bar(self, ctx, ui));
            });

        
        // Tab Dock Area
        let style = theme::get_dock_style(ctx, &theme);
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

