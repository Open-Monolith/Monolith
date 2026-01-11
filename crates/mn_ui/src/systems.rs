use bevy::{platform::collections::HashMap, prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContexts, egui, egui::Rect};

use crate::resize::draw_resize_borders;
use crate::theme::ThemeResource;
use crate::widgets::menubar;
use crate::{dock_state::DockStateResource, viewer::MyTabViewer};
use egui_dock::DockArea;
use mn_core::AppWindowCommand;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut dock_state_res: ResMut<DockStateResource>,
    mut dock_data: ResMut<mn_core::DockData>,
    _window: Single<&mut Window, With<PrimaryWindow>>,
    mut appwindow_writer: MessageWriter<AppWindowCommand>,
    icon_textures: ResMut<mn_core::icons::IconTextures>,
    theme: ResMut<ThemeResource>,
) {
    // Safe guards
    let Ok(ctx) = contexts.ctx_mut() else { return };
    
    let screen_r = ctx.viewport_rect();
    if screen_r.width() < 50.0 || screen_r.height() < 50.0 {
        return;
    }
    dock_data.clear_frame();

    // Windows
    draw_resize_borders(ctx, &mut appwindow_writer);

    // Visuals
    let palette = theme.current();
    let textures = icon_textures.textures.clone();

    // Menubar
    egui::TopBottomPanel::top("main_menu_bar")
        .show_separator_line(false)
        .frame(egui::Frame {
            inner_margin: egui::Margin::symmetric(15, 8),
            fill: palette.bg,
            stroke: egui::Stroke::NONE,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| menubar::menu_bar(ctx, ui, appwindow_writer, &textures));
        });

    // Create dock area with the map
    let mut visible_viewports: HashMap<u32, Rect> = HashMap::new();

    DockArea::new(&mut dock_state_res.dock_state)
        .style(theme.to_dock_style(ctx))
        .show_leaf_collapse_buttons(false)
        .show_close_buttons(false)
        .show_leaf_close_all_buttons(false)
        .show(
            ctx,
            &mut MyTabViewer {
                viewports: &mut visible_viewports,
                icon_textures: &textures,
                theme: &theme,
            },
        );
        
    for (id, rect) in visible_viewports {
        dock_data
            .viewports
            .insert(id, (rect.min.x, rect.min.y, rect.width(), rect.height()));
    }
}
