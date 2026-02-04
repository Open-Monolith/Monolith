use bevy::prelude::*;
use crate::tool::ToolId;

#[derive(Message, Debug, Clone, Copy)]
pub enum ViewportCommand {
    Run(ToolId, u32), // tab_id
}



#[derive(Resource, Default)]
pub struct ActiveTool {
    pub tool: Option<ToolId>,
    pub tab_id: Option<u32>,
}

#[derive(Resource, Default)]
pub struct TwoClickRectState {
    pub start: Option<Vec3>,
}