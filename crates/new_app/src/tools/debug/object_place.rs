use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use new_core::{VisibleViewports};
use new_core::element::{ElementHeader, ElementId};
use new_core::elements::{ElementKind, ElementKindType};
use bevy_egui::egui;
use new_core::elements::element_kindtype_enums::DuctSegmentType;
use crate::editor::selection::picking::Selectable;

use new_core::{GameViewportCamera};


pub fn place_object_here(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    visible_viewports: Res<VisibleViewports>,
    elements: Query<(), With<ElementHeader>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform, &GameViewportCamera)>,
    mut mesh_params: ParamSet<(MeshRayCast, ResMut<Assets<Mesh>>)>
) {
    let ctrl = keys.pressed(KeyCode::ControlLeft);
    if !(ctrl && mouse.just_pressed(MouseButton::Left)) {
        return;
    }

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Some(pane_id) = find_viewport_under_cursor(&visible_viewports, cursor) else {
        return;
    };

    let Some((camera, camera_transform)) = cameras
        .iter()
        .find(|(_,_,tag)| tag.pane_id == pane_id)
        .map(|(cam, tf,_)| (cam, tf))
        else {
            return;
        };  

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
      return;  
    };


    let Some(some_place) = ray.plane_intersection_point(
        Vec3::ZERO,                    // point on plane
        InfinitePlane3d::new(Vec3::Y), // XY plane, normal points up Z
    ) else {
        return;
    };


    // let mut mesh = mesh_params.p1();
    
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
        Mesh3d(mesh_params.p1().add(Sphere::new(0.3).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.55, 0.95),
            ..default()
        })),
        Transform::from_translation(some_place),
        RenderLayers::layer(0), 
        Selectable,
    ));


    
    println!("Hit entity: {:?}", some_place);
}



fn find_viewport_under_cursor(
    visible_viewports: &VisibleViewports,
    cursor: Vec2
) -> Option<u32>{
    let cursor_pos: egui::Pos2 = (cursor.x, cursor.y).into();
    for (&pane_id, rect) in &visible_viewports.rects {
        if rect.contains(cursor_pos) {
            return Some(pane_id);
        }
    }
    None
}