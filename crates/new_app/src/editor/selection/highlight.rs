use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;
use bevy::render::render_resource::Face;

use super::picking::Selected;

#[derive(Component)]
pub struct SelectionOutline;

#[derive(Resource, Clone)]
pub struct OutlineMaterial(pub Handle<StandardMaterial>);

pub fn setup_outline_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.0),
        unlit: true,
        cull_mode: Some(Face::Front),
        depth_bias: 1.0,
        ..Default::default()        
    });

    commands.insert_resource(OutlineMaterial(material));
}

pub fn spawn_outline_for_selected(
    mut commands: Commands,
    outline_material: Res<OutlineMaterial>,
    selected: Query<(Entity, &Mesh3d), Added<Selected>>
) {
    for (entity, mesh) in &selected {
        commands.entity(entity).with_children(|child| {
            child.spawn((
                Name::new("SelectionOutline"),
                Mesh3d(mesh.0.clone()),
                MeshMaterial3d(outline_material.0.clone()),
                Transform::from_scale((Vec3::splat(1.03))),
                SelectionOutline
            ));
        });
    }
}

pub fn remove_outline_for_deselected(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    children: Query<&Children>,
    outlines: Query<(), With<SelectionOutline>>
) {
    for entity in removed.read() {
        if let Ok(kids) = children.get(entity) {
            for child in kids.iter() {
                if outlines.contains(child) {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}