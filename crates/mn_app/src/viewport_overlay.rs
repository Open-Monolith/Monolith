use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use mn_core::{DockData, icons::Icon};

#[derive(Message, Debug, Clone, Copy)]
pub enum ViewportCommand {
    Run(Icon, u32),     // tab_id
}

pub struct ViewportOverlayPlugin;

impl Plugin for ViewportOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ViewportCommand>()
            .add_systems(EguiPrimaryContextPass, viewport_floating_buttons_ui)
            .add_systems(Update, handle_viewport_commands);
    }
}

static ICONS: [Icon; 8] = [
    Icon::ToolModifyAlign,
    Icon::ToolModifyCut,
    Icon::ToolModifyMirror,
    Icon::ToolModifyMove,
    Icon::ToolModifyRotate,
    Icon::ToolModifySelect,
    Icon::ToolModifyTrim,
    Icon::ToolArchitectWall,
];

fn viewport_floating_buttons_ui(
    mut egui_ctxs: EguiContexts,
    dock: Res<DockData>,
    mut ev: MessageWriter<ViewportCommand>,
    // NOTE: if you aren't mutating textures, make this Res<...> not ResMut<...>
    icon_textures: Res<mn_core::icons::IconTextures>,
) {
    let Ok(ctx) = egui_ctxs.ctx_mut() else { return; };

    let textures = icon_textures.textures.clone();


    for (&tab_id, &(x, y, _w, _h)) in dock.viewports.iter() {
        let pad = 8.0;

        // layout tuning
        let gap = 2.0;
        let btn_h = 28.0;
        let btn_w = 70.0;

        // total height of the stack so the area can fit them
        let total_h = ICONS.len() as f32 * btn_h + (ICONS.len().saturating_sub(1) as f32) * gap;

        let area_id = egui::Id::new(("vp_btn_stack", tab_id));
        egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .pivot(egui::Align2::LEFT_TOP)
            .fixed_pos(egui::pos2(x + pad, y + pad))
            .show(ctx, |ui| {
                ui.set_min_size(egui::vec2(btn_w, total_h));

                egui::Frame::new()
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, gap);
                        let image_size = egui::vec2(18.0, 18.0);

                        for (icon) in ICONS.iter() {
                            

                            if ui.add(egui::Button::image((textures[icon], image_size))).clicked() {
                               ev.write(ViewportCommand::Run(*icon, tab_id));
                            }
                        }
                    });
            });
    }
}

fn handle_viewport_commands(mut ev: MessageReader<ViewportCommand>) {
    for cmd in ev.read() {
        match *cmd {
            ViewportCommand::Run(icon, tab_id) => {
                // `icon` is your Icon, `tab_id` is the u32
                match icon {
                    Icon::ToolModifyAlign => {
                        println!("tab_id={tab_id}");
                    }
                    Icon::ToolModifyCut => { /* ... */ }
                    Icon::ToolModifyMirror => { /* ... */ }
                    Icon::ToolModifyMove => { /* ... */ }
                    Icon::ToolModifyRotate => { /* ... */ }
                    Icon::ToolModifySelect => { /* ... */ }
                    Icon::ToolModifyTrim => { /* ... */ }
                    Icon::ToolArchitectWall => { /* ... */ }
                    _ => {}
                }

                // if you need tab_id
                
            }
        }
    }
}
