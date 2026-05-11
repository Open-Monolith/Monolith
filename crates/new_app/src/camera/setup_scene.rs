use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use new_core::element::{ElementHeader, ElementId};
use new_core::elements::{ElementKind, ElementKindType};
use new_core::elements::element_kindtype_enums::DuctSegmentType;
use crate::editor::selection::picking::Selectable;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {                                                                                                                                                                                                                                                                                                                                                                                                                                             
    commands.spawn((
        ElementHeader {
            id: ElementId(1),
            name: Some("new item".to_owned()),
            kind: ElementKind::DuctSegment,
            kind_type: Some(ElementKindType::DuctSegment(
                DuctSegmentType::RIGIDSEGMENT
            )),
            spec_id: Some(ElementId(12)),
            level_id: Some(ElementId(12)),
        },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.20, 0.22),
            ..default()
        })),
        RenderLayers::layer(0),
        Selectable,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.8).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.55, 0.95),
            ..default()
        })),
        Transform::from_xyz(-1.25, 0.8, 0.0),
        RenderLayers::layer(0),
        Selectable,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.65, 0.20),
            ..default()
        })),
        Transform::from_xyz(1.25, 0.5, 0.0),
        RenderLayers::layer(0),
        Selectable,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(0),
    ));
}