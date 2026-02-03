use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use mn_core::DockData;

#[derive(Message, Debug, Clone, Copy)]
pub enum ViewportCommand {
    Run(u32), // tab_id
}

pub struct ViewportOverlayPlugin;

impl Plugin for ViewportOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ViewportCommand>()
            // IMPORTANT: draw egui in the egui pass (so clicks register)
            .add_systems(EguiPrimaryContextPass, viewport_floating_buttons_ui)
            .add_systems(Update, handle_viewport_commands);
    }
}

fn viewport_floating_buttons_ui(
    mut egui_ctxs: EguiContexts,
    dock: Res<DockData>,
    mut ev: MessageWriter<ViewportCommand>,
) {
    let Ok(ctx) = egui_ctxs.ctx_mut() else { return; };

    for (&tab_id, &(x, y, w, _h)) in dock.viewports.iter() {
        let pad = 8.0;
        let id = egui::Id::new(("vp_btn", tab_id));

        egui::Area::new(id)
            .order(egui::Order::Foreground)
            .pivot(egui::Align2::RIGHT_TOP)
            .fixed_pos(egui::pos2(x + w - pad, y + pad))
            .show(ctx, |ui| {
                if ui.button("Run").clicked() {
                    ev.write(ViewportCommand::Run(tab_id));
                }
            });
    }
}

fn handle_viewport_commands(mut ev: MessageReader<ViewportCommand>) {
    for cmd in ev.read() {
        match *cmd {
            ViewportCommand::Run(_tab_id) => {
                println!("Run working");
            }
        }
    }
}
