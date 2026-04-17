use bevy::camera::visibility::RenderLayers;
use bevy::camera::{PerspectiveProjection, Projection, Viewport};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

use new_core::{GameViewportCamera, VisibleViewports};

pub fn sync_viewport_cameras(
    visible_viewports: Res<VisibleViewports>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    existing_tagged: Query<(Entity, &GameViewportCamera)>,
    mut camera_query: Query<(&mut Camera, &mut Projection), With<GameViewportCamera>>,
) {
    let scale_factor = window.scale_factor();
    let win_width = window.physical_width();
    let win_height = window.physical_height();

    let mut existing_map: HashMap<u32, Entity> = HashMap::new();
    for (entity, tag) in &existing_tagged {
        existing_map.insert(tag.pane_id, entity);
    }

    // iterate through existing pane and create/update their 3d camera
    for (&pane_id, rect) in &visible_viewports.rects {
        // convert dock ui rect into physical pixels
        // phys_x, phys_y = viewport x,y origin
        // phys_w, phys_h = viewport width and length (how big)
        let phys_x = (rect.min.x * scale_factor).floor().max(0.0) as u32;
        let phys_y = (rect.min.y * scale_factor).floor().max(0.0) as u32;
        let mut phys_w = (rect.width() * scale_factor).ceil().max(0.0) as u32;
        let mut phys_h = (rect.height() * scale_factor).ceil().max(0.0) as u32;

        // clamp viewport size so does not go outside window
        if phys_x + phys_w > win_width {
            phys_w = win_width.saturating_sub(phys_x);
        }
        if phys_y + phys_h > win_height {
            phys_h = win_height.saturating_sub(phys_y);
        }

        // if viewport panel has no size, do not render
        if phys_w == 0 || phys_h == 0 {
            if let Some(&entity) = existing_map.get(&pane_id) {
                if let Ok((mut camera, _)) = camera_query.get_mut(entity) {
                    camera.is_active = false;
                    camera.viewport = None;
                }
            }
            continue;
        }

        // Create the bevy viewport
        let aspect_ratio = phys_w as f32 / phys_h as f32;
        let viewport = Viewport {
            physical_position: UVec2::new(phys_x, phys_y),
            physical_size: UVec2::new(phys_w, phys_h),
            depth: 0.0..1.0,
        };

        // update existing camera
        // 1. if existing, update size, order, aspect ratio, and turn active
        // 2. if broken, delete and recreate cleanly
        // 3. no camera, then spawn new
        if let Some(&entity) = existing_map.get(&pane_id) {
            if let Ok((mut camera, mut projection)) = camera_query.get_mut(entity) {
                // negative so viewport camera stays below ui camera (1) order
                camera.order = -(pane_id as isize);
                camera.is_active = true;
                camera.viewport = Some(viewport);

                if let Projection::Perspective(p) = &mut *projection {
                    p.aspect_ratio = aspect_ratio;
                }
            } else {
                commands.entity(entity).despawn();
                spawn_viewport_camera(&mut commands, pane_id, viewport, aspect_ratio);
            }
        } else {
            spawn_viewport_camera(&mut commands, pane_id, viewport, aspect_ratio);
        }
    }

    // turn off camera if pane does not exists
    for (pane_id, entity) in existing_map {
        if visible_viewports.rects.contains_key(&pane_id) {
            continue;
        }

        if let Ok((mut camera, _)) = camera_query.get_mut(entity) {
            camera.is_active = false;
            camera.viewport = None;
        }
    }
}

fn spawn_viewport_camera(
    commands: &mut Commands,
    pane_id: u32,
    viewport: Viewport,
    aspect_ratio: f32,
) {
    commands.spawn((
        Name::new(format!("GameViewportCamera({pane_id})")),
        Camera3d::default(),
        Camera {
            order: -(pane_id as isize),
            is_active: true,
            viewport: Some(viewport),
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: 60f32.to_radians(),
            aspect_ratio,
            near: 0.1,
            far: 10_000.0,
            ..default()
        }),
        Transform::from_xyz(4.0, 3.0, 6.0).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
        RenderLayers::layer(0),
        GameViewportCamera { pane_id },
    ));
}