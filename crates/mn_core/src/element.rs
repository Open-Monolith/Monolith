use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ElementId(pub Uuid);

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(pub Uuid);

#[derive(Bundle)]
pub struct ElementRenderBundle {
    pub id: ElementId,
    pub type_id: TypeId,
    pub name: Name,

    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
}

// Params
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ParamKey(pub u32);

#[derive(Clone, Debug)]
pub enum ParamValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

#[derive(Component, Default, Clone, Debug)]
pub struct ParamBag {
    pub values: Vec<(ParamKey, ParamValue)>,
}