use bevy::prelude::*;
use mn_core::commands::{ActiveTool, TwoClickRectState, ViewportCommand};
use mn_core::tool::ToolId;

use crate::tools::architect_wall::{WallAssets, WallGhostEntity, ensure_wall_ghost};

pub struct AppCommands;

// impl Plugin for AppCommands {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, handle_viewport_commands)
//            .add_systems(PostUpdate, two_click_rect_system.after(TransformSystems::Propagate))

//     }
// }

pub fn handle_viewport_commands(
    mut ev: MessageReader<ViewportCommand>,
    mut active: ResMut<ActiveTool>,
    mut state: ResMut<TwoClickRectState>,
    assets: Res<WallAssets>,
    mut ghost_ent: ResMut<WallGhostEntity>,
    mut commands: Commands,
) {
    for cmd in ev.read() {
        match *cmd {
            ViewportCommand::Run(tool_id, tab_id) => match tool_id {
                ToolId::ArchitectWall => {
                    active.tool = Some(tool_id);
                    active.tab_id = Some(tab_id);

                    state.start = None;
                    ensure_wall_ghost(&mut commands, &assets, &mut ghost_ent);
                    println!("Active tool set to {tool_id:?} on tab_id={tab_id}");
                }
                ToolId::ModifySelect => todo!(),
                ToolId::ModifyMove => todo!(),
                ToolId::ModifyRotate => todo!(),
                ToolId::ModifyAlign => todo!(),
                ToolId::ModifyTrim => todo!(),
                ToolId::ModifyMirror => todo!(),
                ToolId::ModifyCut => todo!(),
            },
        }
    }
}
