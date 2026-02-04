use bevy::camera::{PerspectiveProjection, Projection, Viewport, visibility::RenderLayers};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use std::collections::HashSet;

use mn_core::DockData;
use crate::camera_controls::TabViewportCamera;

pub fn update_viewport_system(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    existing_tagged: Query<(Entity, &TabViewportCamera)>,
    mut camera_query: Query<&mut Camera>,
) {
    // Build a map of existing tab_id -> entity
    let mut existing_map: HashMap<u32, Entity> = HashMap::new();
    for (entity, tag) in existing_tagged.iter() {
        existing_map.insert(tag.tab_id, entity);
    }

    // Which tab ids we need this frame
    let tab_ids_needed: HashSet<u32> = dock_data.viewports.keys().copied().collect();

    // Window physical size and scale
    let win_width = window.physical_width();
    let win_height = window.physical_height();
    let scale_factor = window.scale_factor();

    for (&tab_id, &(x, y, w, h)) in dock_data.viewports.iter() {
        // Convert logical egui coordinates -> physical pixels
        let phys_x: u32 = (x * scale_factor).max(0.0) as u32;
        let phys_y: u32 = (y * scale_factor).max(0.0) as u32;
        let mut phys_w: u32 = (w * scale_factor).max(0.0) as u32;
        let mut phys_h: u32 = (h * scale_factor).max(0.0) as u32;

        // clamp to window
        if phys_x + phys_w > win_width {
            phys_w = win_width.saturating_sub(phys_x);
        }
        if phys_y + phys_h > win_height {
            phys_h = win_height.saturating_sub(phys_y);
        }
        if phys_w == 0 || phys_h == 0 {
            continue;
        }

        let viewport = Viewport {
            physical_position: UVec2::new(phys_x, phys_y),
            physical_size: UVec2::new(phys_w, phys_h),
            ..default()
        };

        if let Some(&entity) = existing_map.get(&tab_id) {
            // update existing camera's viewport & order
            if let Ok(mut cam) = camera_query.get_mut(entity) {
                cam.viewport = Some(viewport);
                cam.is_active = true;
                // Camera.order is an `isize` — using negative values to create unique ordering is fine.
                cam.order = -(tab_id as isize);
            } else {
                // camera missing? despawn the tagged entity (it may have been partially destroyed)
                commands.entity(entity).despawn();
            }
        } else {
            // create a new Camera3d for this tab (perspective)
            let proj: Projection = Projection::from(PerspectiveProjection {
                fov: 60f32.to_radians(),
                aspect_ratio: phys_w as f32 / phys_h as f32,
                near: 0.1,
                far: 10000.0,
                ..default()
            });

            let transform = Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);

            commands.spawn((
                Camera3d::default(),
                MeshPickingCamera,
                Pickable::default(),
                Camera {
                    order: -(tab_id as isize),
                    viewport: Some(viewport),
                    ..default()
                },
                proj,
                transform,
                GlobalTransform::default(),
                RenderLayers::layer(0),
                TabViewportCamera { tab_id },
                crate::camera_controls::BimOrbitCamera::default(),
                // crate::camera_controls::DefaultPivot(Vec3::ZERO),
            ));
        }
    }

    // Despawn cameras whose tab ids are no longer present
    for (existing_tab_id, entity) in existing_map.into_iter() {
        if !tab_ids_needed.contains(&existing_tab_id) {
            commands.entity(entity).despawn();
        }
    }
}
