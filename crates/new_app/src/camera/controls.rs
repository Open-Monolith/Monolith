use bevy::prelude::*;
use bevy::input::mouse::{
    AccumulatedMouseMotion,
    AccumulatedMouseScroll
};
use bevy::window::PrimaryWindow;

use new_core::{GameViewportCamera, VisibleViewports};

#[derive(Component, Debug, Clone)]
pub struct ViewportOrbitCamera {
    pub pivot: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,

    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    
    pub min_distance: f32,
    pub max_distance: f32,
}

impl Default for ViewportOrbitCamera {
    fn default() -> Self {
        Self {
            pivot: Vec3::new(0.0, 0.7, 0.0),
            yaw: 0.0,
            pitch: -0.6,
            distance: 6.0,

            orbit_sensitivity: 0.005,
            pan_sensitivity: 0.002,
            zoom_sensitivity: 0.1,

            min_distance: 0.05,
            max_distance: 100_000.0,
        }
    }
}

impl ViewportOrbitCamera {
    pub fn from_eye_and_target(eye: Vec3, target: Vec3) -> Self {
        let offset = eye - target;
        let distance = offset.length().max(0.001);
        let horizontal = (offset.x * offset.x + offset.z * offset.z).sqrt().max(0.0001);

        let yaw = offset.x.atan2(offset.z);
        let pitch = (-offset.y).atan2(horizontal);

        Self {
            pivot: target,
            yaw,
            pitch,
            distance,
            ..Default::default()
        }
    }

    pub fn apply_to_transform(&self, transform: &mut Transform) {
        let rotation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
        let offset = rotation * Vec3::new(0.0, 0.0, self.distance);

        transform.translation = self.pivot + offset;
        transform.look_at(self.pivot, Vec3::Y);
    }
}


pub fn viewport_camera_controls_system(
    visible_viewports: Res<VisibleViewports>,
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut cameras: Query<(&GameViewportCamera, &mut Transform, &mut ViewportOrbitCamera)>
) {
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Some(active_pane_id) = find_viewport_under_cursor(&visible_viewports, cursor) else {
        return;
    };

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let orbiting = mouse_buttons.pressed(MouseButton::Middle) && !shift;
    let panning = mouse_buttons.pressed(MouseButton::Middle) && shift;

    let moved = mouse_motion.delta != Vec2::ZERO;
    let scrolled = mouse_scroll.delta.y != 0.0;

    if !scrolled && !(moved && (orbiting || panning)) {
        return;
    }

    for (tag, mut transform, mut orbit) in &mut cameras {
        if tag.pane_id != active_pane_id {
            continue;
        }

        if scrolled {
            let zoom_factor = (1.0 - mouse_scroll.delta.y * orbit.zoom_sensitivity).max(0.01);
            orbit.distance = (orbit.distance * zoom_factor).clamp(orbit.min_distance, orbit.max_distance);
        }

        if moved {
            let delta = mouse_motion.delta;

            if orbiting {
                orbit.yaw -= delta.x * orbit.orbit_sensitivity;
                orbit.pitch -= delta.y * orbit.orbit_sensitivity;

                let limit = 1.54; // about 88 degrees
                orbit.pitch = orbit.pitch.clamp(-limit, limit);
            } else if panning {
                let rotation =
                    Quat::from_rotation_y(orbit.yaw) * Quat::from_rotation_x(orbit.pitch);

                let right = rotation * Vec3::X;
                let up = rotation * Vec3::Y;

                let pan_scale = orbit.distance * orbit.pan_sensitivity;
                orbit.pivot += (-right * delta.x + up * delta.y) * pan_scale;
            }
        }

        orbit.apply_to_transform(&mut transform);
        break;
    }
}

fn find_viewport_under_cursor(
    visible_viewports: &VisibleViewports,
    cursor: Vec2
) -> Option<u32>{
    for (&pane_id, rect) in &visible_viewports.rects {
        // Check if cursor within a pane_id's rect boundary
        if cursor.x >= rect.min.x 
            && cursor.x <= rect.max.x
            && cursor.y >= rect.min.y
            && cursor.y <= rect.max.y
            {
                return Some(pane_id)
            }
    }

    None
}