use bevy::{
    camera::{CameraOutputMode, Viewport, visibility::RenderLayers},
    prelude::*,
    render::render_resource::BlendState,
    window::{PrimaryWindow, WindowMode},
};
use bevy_egui::{
    EguiGlobalSettings, EguiPlugin,
    PrimaryEguiContext,
};
use mn_egui::{AppWindowCommand, MonolithUIPlugin};
use mn_core::{DockData};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Monolith BIM".into(),
                // This removes the OS title bar and borders
                decorations: false, 
                // distinct from "maximized", this centers it nicely on startup
                position: WindowPosition::Centered(MonitorSelection::Primary), 
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(MonolithUIPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(PostUpdate, update_viewport_system)
        .add_systems(Update, windows_control_system)
        .run();
}

fn windows_control_system(
    mut reader: MessageReader<AppWindowCommand>, // <- read messages
    mut app_exit_events: MessageWriter<AppExit>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {

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
                    window.mode = WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary);
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

fn update_viewport_system(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    // Query specifically for the World Camera
    mut camera: Single<&mut Camera, (Without<PrimaryEguiContext>, With<Camera2d>)>,
) {
    // 1. Get the CURRENT physical window dimensions
    let win_width = window.physical_width();
    let win_height = window.physical_height();

    if let Some((x, y, w, h)) = dock_data.viewport_logical {
        let scale_factor = window.scale_factor();

        let phys_x = (x * scale_factor).max(0.0) as u32;
        let phys_y = (y * scale_factor).max(0.0) as u32;
        let mut phys_w = (w * scale_factor).max(0.0) as u32;
        let mut phys_h = (h * scale_factor).max(0.0) as u32;

        if phys_x + phys_w > win_width {
            phys_w = win_width.saturating_sub(phys_x);
        }
        if phys_y + phys_h > win_height {
            phys_h = win_height.saturating_sub(phys_y);
        }

        if phys_w > 0 && phys_h > 0 {
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(phys_x, phys_y),
                physical_size: UVec2::new(phys_w, phys_h),
                ..default()
            });
            camera.is_active = true;
            return;
        }
    }

    // Disable camera if no valid dock exists
    camera.is_active = false;
}




fn setup_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Disable automatic primary context creation
    egui_global_settings.auto_create_primary_context = false;

    // Spawn world mesh and put it on render layer 0
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_translation(Vec3::new(-150.0, 0.0, 0.0)),
        RenderLayers::layer(0), // world entities on layer 0
    ));

    // The camera that will render your world into the egui-tab viewport.
    // This camera has no PrimaryEguiContext, so your existing viewport-update query will target it.
    commands.spawn((
        Camera2d,                       // the 2D marker (no bundle)
        Camera {
            // start with full window; update_viewport_system will set Camera.viewport = Some(...)
            order: 0,
            ..default()
        },
        RenderLayers::layer(0),         // this camera renders layer 0 (the world)
    ));

    // Egui camera (renders UI). Put it on a different render layer so it doesn't draw world entities.
    commands.spawn((
        PrimaryEguiContext,
        Camera2d, // make it a 2d camera component as you had before
        Camera {
            order: 1,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderLayers::layer(1), // different layer -> won't render world entities on layer 0
    ));
}


