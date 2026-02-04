use bevy::prelude::*;
use crate::icons::Icon;
use enum_map::{Enum, EnumMap, enum_map};

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolId {
    ModifySelect,
    ModifyMove,
    ModifyRotate,
    ModifyAlign,
    ModifyTrim,
    ModifyMirror,
    ModifyCut,
    ArchitectWall,
}

#[derive(Debug, Clone, Copy)]
pub struct ToolDef {
    pub id: ToolId,
    pub icon: Icon,
    pub label: &'static str,
    pub tooltip: &'static str
}

#[derive(Resource)]
pub struct ToolRegistry {
    pub map: EnumMap<ToolId, ToolDef>,
}

impl FromWorld for ToolRegistry {
    fn from_world(_: &mut World) -> Self {
        let map = enum_map! {
            ToolId::ModifySelect  => ToolDef { id: ToolId::ModifySelect, icon: Icon::ToolModifySelect, label: "Select", tooltip: "Select elements" },
            ToolId::ModifyMove    => ToolDef { id: ToolId::ModifyMove,   icon: Icon::ToolModifyMove,   label: "Move",   tooltip: "Move elements" },
            ToolId::ModifyRotate  => ToolDef { id: ToolId::ModifyRotate, icon: Icon::ToolModifyRotate, label: "Rotate", tooltip: "Rotate elements" },
            ToolId::ModifyAlign   => ToolDef { id: ToolId::ModifyAlign,  icon: Icon::ToolModifyAlign,  label: "Align",  tooltip: "Align elements" },
            ToolId::ModifyTrim    => ToolDef { id: ToolId::ModifyTrim,   icon: Icon::ToolModifyTrim,   label: "Trim",   tooltip: "Trim/extend" },
            ToolId::ModifyMirror  => ToolDef { id: ToolId::ModifyMirror, icon: Icon::ToolModifyMirror, label: "Mirror", tooltip: "Mirror elements" },
            ToolId::ModifyCut     => ToolDef { id: ToolId::ModifyCut,    icon: Icon::ToolModifyCut,    label: "Cut",    tooltip: "Cut geometry" },
            ToolId::ArchitectWall => ToolDef { id: ToolId::ArchitectWall,icon: Icon::ToolArchitectWall,label: "Wall",   tooltip: "Create walls" },
        };
        Self { map }
    }
}

impl ToolRegistry {
    #[inline]
    pub fn def(&self, id: ToolId) -> ToolDef {
        self.map[id]
    }
}

pub const MODIFY_TOOLS: &[ToolId] = &[
    ToolId::ModifySelect,
    ToolId::ModifyMove,
    ToolId::ModifyRotate,
    ToolId::ModifyAlign,
    ToolId::ModifyTrim,
    ToolId::ModifyMirror,
    ToolId::ModifyCut,
];

pub const ARCHITECT_TOOLS: &[ToolId] = &[
    ToolId::ArchitectWall,
];