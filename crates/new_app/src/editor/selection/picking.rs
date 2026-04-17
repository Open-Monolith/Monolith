use bevy::input::mouse;
use bevy::prelude::*;
use bevy::picking::mesh_picking::ray_cast::{
    MeshRayCast, MeshRayCastSettings, RayCastVisibility
};
use bevy::window::PrimaryWindow;

use bevy_egui::egui;
use new_core::{GameViewportCamera, VisibleViewports};

#[derive(Resource, Default)]
pub struct SelectionStae {
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
    mut commands: Commands
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Some(pane_id) = find_viewport_under_cursor(&visible_viewports, cursor) else {
        // clear_selection
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
      // clear_selection
      return;  
    };

    println!("{}", cursor);
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