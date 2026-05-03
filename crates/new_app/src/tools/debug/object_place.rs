use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use new_core::VisibleViewports;

use bevy_egui::egui;
use new_core::{GameViewportCamera, VisibleViewports};


pub fn place_object_here(
    visible_viewports: Res<VisibleViewports>,

    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform, &GameViewportCamera)>,

) {
    if  keys.pressed(KeyCode::ControlLeft) && !mouse.just_pressed(MouseButton::Left) {
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