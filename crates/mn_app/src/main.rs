use bevy::{
    camera::{CameraOutputMode, ClearColorConfig, Projection, PerspectiveProjection, Viewport, visibility::RenderLayers},
    prelude::*,
    render::render_resource::BlendState,
    window::{PrimaryWindow, WindowMode, WindowPosition, MonitorSelection},
    mesh::Mesh3d,
    platform::collections::HashMap
};
use bevy::pbr::{StandardMaterial, MeshMaterial3d};
use bevy_egui::{EguiGlobalSettings, EguiPlugin, PrimaryEguiContext};

use mn_core::{AppWindowCommand, DockData}; // project-local: I could not validate these

use std::collections::HashSet;

#[derive(Component)]
struct TabViewportCamera {
    pub tab_id: u32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_plugins(
            DefaultPlugins.set(bevy::window::WindowPlugin {
                primary_window: Some(Window {
                    title: "Monolith BIM".into(),
                    decorations: false,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(mn_ui::icons::IconsPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(mn_ui::MonolithUIPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(PostUpdate, update_viewport_system)
        .add_systems(Update, windows_control_system)
        .run();
}

fn windows_control_system(
    mut reader: MessageReader<AppWindowCommand>, // project-local message reader
    mut app_exit_events: MessageWriter<AppExit>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // get the primary window (Single param alternative works too, but this mirrors your code)
    let mut window = match window_query.single_mut() {
        Ok(w) => w,
        Err(_) => return,
    };

    for cmd in reader.read() {
        match cmd {
            AppWindowCommand::Shutdown => {
                app_exit_events.write(AppExit::Success);
            }
            AppWindowCommand::ToggleMaximize => {
                if window.mode != WindowMode::Windowed {
                    window.mode = WindowMode::Windowed;
                } else {
                    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
                }
            }
            AppWindowCommand::Minimize => {
                window.set_minimized(true);
            }
            AppWindowCommand::StartMove => {
                window.start_drag_move();
            }
            AppWindowCommand::StartResize(octant) => {
                window.mode = WindowMode::Windowed;
                window.start_drag_resize(*octant);
            }
        }
    }
}

fn setup_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // disable automatic single primary context; we'll create it manually
    egui_global_settings.auto_create_primary_context = false;

    // spawn primary egui camera/context (2D camera used for egui compositing)
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            order: 1,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));

    // spawn a simple 3D sphere in render layer 0
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh().ico(5).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2).into(),
            ..Default::default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
        RenderLayers::layer(0),
    ));
}

/// Update cameras that render to each dock viewport (one Camera3d per viewport/tab).
fn update_viewport_system(
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
            let proj = Projection::from(PerspectiveProjection {
                fov: 60f32.to_radians(),
                aspect_ratio: phys_w as f32 / phys_h as f32,
                near: 0.1,
                far: 10000.0,
            });

            let transform = Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);

            commands.spawn((
                Camera3d::default(),
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
