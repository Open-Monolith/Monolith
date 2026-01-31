use bevy::input::mouse::{
    AccumulatedMouseMotion, AccumulatedMouseScroll, accumulate_mouse_motion_system,
    accumulate_mouse_scroll_system,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::DockData;

#[derive(Component, Copy, Clone, Debug)]
pub struct TabViewportCamera {
    pub tab_id: u32,
}

#[derive(Component, Debug, Clone)]
pub struct BimOrbitCamera {
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

impl Default for BimOrbitCamera {
    fn default() -> Self {
        Self {
            pivot: Vec3::ZERO,
            yaw: 0.0,
            pitch: -0.6,
            distance: 6.0,
            orbit_sensitivity: 0.005,
            pan_sensitivity: 0.002,
            zoom_sensitivity: 0.15,
            min_distance: 0.01,
            max_distance: 100_000.0,
        }
    }
}

pub struct BimCameraControlsPlugin;

impl Plugin for BimCameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                accumulate_mouse_motion_system,
                accumulate_mouse_scroll_system,
            ),
        );

        app.add_systems(Update, bim_camera_controls_system);
    }
}

fn bim_camera_controls_system(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,

    mut q: Query<(
        &TabViewportCamera,
        &mut Transform,
        &mut BimOrbitCamera,
        &mut Projection,
    )>,
) {
    let window = window.into_inner();

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    // Which viewport rect is the mouse inside?
    let Some(active_tab_id) = find_tab_under_cursor(dock_data.as_ref(), cursor) else {
        return;
    };

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // Control ONLY the camera for that tab_id
    for (tag, mut tf, mut cam, mut proj) in q.iter_mut() {
        if tag.tab_id != active_tab_id {
            continue;
        }

        // Keep the 3D projection aspect ratio correct for the dock viewport.
        if let Some((_, _, w, h)) = dock_data.viewports.get(&active_tab_id).copied() {
            if w > 1.0 && h > 1.0 {
                if let Projection::Perspective(p) = &mut *proj {
                    p.aspect_ratio = w / h;
                }
            }
        }

        // Zoom (wheel)
        if mouse_scroll.delta.y != 0.0 {
            let zoom_factor = (1.0 - mouse_scroll.delta.y * cam.zoom_sensitivity).max(0.01);
            cam.distance = (cam.distance * zoom_factor).clamp(cam.min_distance, cam.max_distance);
        }

        // Orbit or Pan (MMB drag)
        if mouse_buttons.pressed(MouseButton::Middle) {
            let delta = mouse_motion.delta;

            let rot = Quat::from_rotation_y(cam.yaw) * Quat::from_rotation_x(cam.pitch);
            let right = rot * Vec3::X;
            let up = rot * Vec3::Y;

            if shift {
                // Pan: move pivot in camera plane
                let pan_scale = cam.distance * cam.pan_sensitivity;
                cam.pivot += (-right * delta.x + up * delta.y) * pan_scale;
            } else {
                // Orbit
                cam.yaw -= delta.x * cam.orbit_sensitivity;
                cam.pitch -= delta.y * cam.orbit_sensitivity;

                let limit = 1.54; // ~88 deg
                cam.pitch = cam.pitch.clamp(-limit, limit);
            }
        }

        // Apply transform from pivot + yaw/pitch + distance
        let rot = Quat::from_rotation_y(cam.yaw) * Quat::from_rotation_x(cam.pitch);
        let offset = rot * Vec3::new(0.0, 0.0, cam.distance);

        tf.translation = cam.pivot + offset;
        tf.look_at(cam.pivot, Vec3::Y);

        break;
    }
}

fn find_tab_under_cursor(dock_data: &DockData, cursor: Vec2) -> Option<u32> {
    for (&tab_id, &(x, y, w, h)) in dock_data.viewports.iter() {
        if cursor.x >= x && cursor.x <= x + w && cursor.y >= y && cursor.y <= y + h {
            return Some(tab_id);
        }
    }
    None
}
