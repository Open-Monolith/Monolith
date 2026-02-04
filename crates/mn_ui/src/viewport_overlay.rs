use crate::theme::ThemeResource;
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, CornerRadius},
};
use mn_core::{
    DockData,
    commands::ViewportCommand,
    tool::{ARCHITECT_TOOLS, MODIFY_TOOLS, ToolRegistry},
};

const TOOL_LEN: usize = ARCHITECT_TOOLS.len() + MODIFY_TOOLS.len();

pub struct ViewportOverlayPlugin;

impl Plugin for ViewportOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ViewportCommand>()
            .add_systems(EguiPrimaryContextPass, viewport_floating_buttons_ui);
    }
}

fn viewport_floating_buttons_ui(
    mut egui_ctxs: EguiContexts,
    dock: Res<DockData>,
    tools: Res<ToolRegistry>,
    mut ev: MessageWriter<ViewportCommand>,
    icon_textures: Res<mn_core::icons::IconTextures>,
    theme: Res<ThemeResource>,
) {
    let Ok(ctx) = egui_ctxs.ctx_mut() else {
        return;
    };

    let textures = &icon_textures.textures;
    let palette = theme.current();

    for (&tab_id, &(x, y, _w, _h)) in dock.viewports.iter() {
        let pad = 8.0;

        // layout tuning
        let gap = 0.8;
        let btn_h = 28.0;
        let btn_w = 70.0;

        // total height of the stack so the area can fit them
        let total_h = TOOL_LEN as f32 * btn_h + (TOOL_LEN.saturating_sub(1) as f32) * gap;

        let area_id = egui::Id::new(("vp_btn_stack", tab_id));
        egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .pivot(egui::Align2::LEFT_TOP)
            .fixed_pos(egui::pos2(x + pad, y + pad))
            .show(ctx, |ui| {
                ui.set_min_size(egui::vec2(btn_w, total_h));

                egui::Frame::NONE
                    .fill(egui::Color32::TRANSPARENT)
                    .corner_radius(CornerRadius::same(5))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, gap);
                        let image_size = egui::Vec2::splat(23.);
                        ui.spacing_mut().button_padding = egui::Vec2::splat(6.);
                        for &tool_id in MODIFY_TOOLS.iter().chain(ARCHITECT_TOOLS.iter()) {
                            let tool_def = tools.def(tool_id);
                            if ui
                                .add(
                                    egui::Button::image((textures[&tool_def.icon], image_size))
                                        .fill(palette.panel)
                                        .corner_radius(CornerRadius::same(2)),
                                )
                                .clicked()
                            {
                                ev.write(ViewportCommand::Run(tool_def.id, tab_id));
                            }
                        }
                    });
            });
    }
}
