use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui, EguiContexts,
};
use egui_dock::DockArea;

use mn_core::AppWindowCommand;
use crate::theme::{get_dock_style, Theme};
use crate::{dock_state::DockStateResource, viewer::MyTabViewer};
use crate::widgets::{menubar};
use crate::resize::draw_resize_borders;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut dock_state_res: ResMut<DockStateResource>, 
    mut dock_data: ResMut<mn_core::DockData>,
    _window: Single<&mut Window, With<PrimaryWindow>>,
    mut appwindow_writer: MessageWriter<AppWindowCommand>, // system param
    ) {
    
    
    
    let Ok(ctx) = contexts.ctx_mut() else { return };
    let screen_r = ctx.viewport_rect();
    if screen_r.width() < 50.0 || screen_r.height() < 50.0 {
        return;
    }
    dock_data.clear_frame();
    
    let mut viewport_rect: Option<egui::Rect> = None;
    
    draw_resize_borders(ctx, &mut appwindow_writer);
        
    ctx.set_visuals(egui::Visuals::dark());
    let theme: Theme = Theme::from_ctx(ctx);

    egui::TopBottomPanel::top("main_menu_bar")
        .show_separator_line(false)
        .frame(egui::Frame {
            inner_margin: egui::Margin::symmetric(15, 8),
             fill: theme.bg,
            stroke: egui::Stroke::NONE,
            // corner_radius: egui::CornerRadius {
            //     nw: 10, // Top-Left
            //     ne: 10, // Top-Right
            //     sw: 0,  // Bottom-Left
            //     se: 0,  // Bottom-Right
            // },
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                menubar::menu_bar(ctx, ui, appwindow_writer)
            });
        });

    let style = get_dock_style(ctx, &theme);
    DockArea::new(&mut dock_state_res.dock_state)
        .style(style)
        .show_leaf_collapse_buttons(false)
        .show_close_buttons(false)
        .show_leaf_close_all_buttons(false)
        .show(ctx, 
            &mut MyTabViewer {
                viewport_out: &mut viewport_rect
            });
    
    if let Some(rect) = viewport_rect {
        let lt = rect.left_top();
        let sz = rect.size();
        dock_data.set_viewport_from_logical((lt.x, lt.y), (sz.x, sz.y));
    }
}
