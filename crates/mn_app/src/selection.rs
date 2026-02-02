use bevy::prelude::*;
use bevy::picking::prelude::*;
use bevy::picking::mesh_picking::{
    MeshPickingPlugin, MeshPickingSettings, MeshPickingCamera
};
use bevy::pbr::MeshMaterial3d;

#[derive(Resource, Default)]
pub struct Selected(pub Option<Entity>);

#[deri]