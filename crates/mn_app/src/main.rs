
// external crates
use uuid::Uuid;

// bevy
use bevy::prelude::*;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::mesh::Mesh3d;
use bevy::camera::{
    CameraOutputMode, ClearColorConfig, visibility::RenderLayers,
};
use bevy::render::render_resource::BlendState;
use bevy::window::{MonitorSelection, WindowPosition};

// bevy_egui
use bevy_egui::{EguiGlobalSettings, EguiPlugin, EguiStartupSet, PrimaryEguiContext};
use bevy::transform::TransformSystems;

// local crate
use crate::selection::Selectable;
use crate::commands::handle_viewport_commands;
use crate::viewport::update_viewport_system;
use crate::window::windows_control_system;

// workspace crate
use mn_core::{DockData, element::ElementId};
use mn_core::tool::ToolRegistry;
use mn_core::commands::{ActiveTool, TwoClickRectState};

// modules
mod camera_controls;
mod selection;
mod world_grid;
mod tools;
mod commands;
mod viewport;
mod window;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .init_resource::<ToolRegistry>()
        .init_resource::<ActiveTool>()
        .init_resource::<TwoClickRectState>()
        .add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
            primary_window: Some(Window {
                title: "Monolith BIM".into(),
                decorations: false,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        
        // Plugins
        .add_plugins(EguiPlugin::default())
        .add_plugins(mn_ui::MonolithUIPlugin)
        .add_plugins(world_grid::WorldGridPlugin)
        .add_plugins(crate::camera_controls::BimCameraControlsPlugin)
        .add_plugins(crate::selection::SelectionPlugin)
        .add_plugins(crate::tools::architect_wall::ArchitectWallPlugin)
        // Systems
        .add_systems(Startup, (setup_system, test_system).before(EguiStartupSet::InitContexts))
        .add_systems(PostUpdate, update_viewport_system.before(TransformSystems::Propagate))
        .add_systems(Update, windows_control_system)
        .add_systems(Update, handle_viewport_commands)
        // .add_systems(PostUpdate, two_click_rect_system.after(TransformSystems::Propagate))
        .run();
}

fn setup_system(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
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
}

fn test_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2),
        ..default()
    });

    commands
        .spawn((
            ElementId(Uuid::new_v4()),
            Name::new("Element 1"),
            Transform::from_xyz(2., 2., 0.),
            GlobalTransform::default(),
            Mesh3d(mesh),
            MeshMaterial3d(material.clone()),
            RenderLayers::layer(0),
            Selectable,
        ));
    commands
        .spawn((
            ElementId(Uuid::new_v4()),
            Name::new("Element 1=2"),
            Transform::from_xyz(0., 0., 0.),
            GlobalTransform::default(),
            Mesh3d(meshes.add(Cuboid::from_length(1.0))),
            MeshMaterial3d(material.clone()),
            RenderLayers::layer(0),
            Selectable,
        ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(0),
    ));
}
