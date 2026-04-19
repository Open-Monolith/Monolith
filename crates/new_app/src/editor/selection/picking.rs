use bevy::prelude::*;
use bevy::picking::mesh_picking::ray_cast::{
    MeshRayCast, MeshRayCastSettings, RayCastVisibility
};
use bevy::window::PrimaryWindow;

use bevy_egui::egui;
use new_core::{GameViewportCamera, VisibleViewports};

#[derive(Resource, Default)]
pub struct SelectionState {
    pub current: Option<Entity>
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]    
pub struct Selected;

pub fn select_with_left_click(
    visible_viewports: Res<VisibleViewports>,
    window: Single<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform, &GameViewportCamera)>,
    selectables: Query<(), With<Selectable>>,
    mut selection: ResMut<SelectionState>,
    exists: Query<(), ()>,
    mut ray_cast: MeshRayCast,
    mut commands: Commands
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Some(pane_id) = find_viewport_under_cursor(&visible_viewports, cursor) else {
        clear_selection(&mut selection, &exists, &mut commands);
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
      clear_selection(&mut selection, &exists, &mut commands);
      return;  
    };

    let filter = |entity: Entity| selectables.contains(entity);

    let settings = MeshRayCastSettings::default()
        .with_filter(&filter)
        .with_visibility(RayCastVisibility::Visible);

    let hit_entity = ray_cast
        .cast_ray(ray, &settings)
        .first()
        .map(|(entity, _hit)| *entity);
    
    set_selection(
        &mut selection,
        hit_entity,
        &exists,
        &mut commands
    );

    println!("{}", cursor);
}

fn set_selection(
    selection: &mut SelectionState,
    new_entity: Option<Entity>,
    exists: &Query<(), ()>,
    commands: &mut Commands
) {
    if selection.current == new_entity {
        return;
    }

    if let Some(old) = selection.current.take() {
        if exists.get(old).is_ok() {
            commands.entity(old).remove::<Selected>();
        }
    }

    if let Some(entity) = new_entity {
        if exists.get(entity).is_ok() {
            commands.entity(entity).insert(Selected);
            selection.current = Some(entity);
        }
    }
}


fn clear_selection(
    selection: &mut SelectionState,
    exists: &Query<(), ()>,
    commands: &mut Commands,
) {
    set_selection(selection, None, exists, commands);
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