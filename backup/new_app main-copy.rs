use bevy::camera::{
    CameraOutputMode, ClearColorConfig, Viewport, visibility::RenderLayers,
};
use bevy::prelude::*;
use bevy::render::render_resource::BlendState;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    EguiStartupSet, PrimaryEguiContext,
};
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};

#[derive(Clone, Copy, Debug)]
enum PaneKind {
    Console,
    Properties,
    Viewport,
}

#[derive(Clone, Debug)]
struct Pane {
    kind: PaneKind,
}

#[derive(Resource)]
struct DockTree {
    tree: Tree<Pane>,
    viewport_tile: TileId,
}

#[derive(Resource, Default)]
struct UiState {
    console_filter: String,
    props_enabled: bool,
}

#[derive(Resource, Default)]
struct ViewportState {
    rect_points: Option<egui::Rect>,
    pixels_per_point: f32,
}

#[derive(Component)]
struct GameViewportCamera;

struct TreeBehavior<'a> {
    ui_state: &'a mut UiState,
}

impl<'a> Behavior<Pane> for TreeBehavior<'a> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane.kind {
            PaneKind::Console => "Console".into(),
            PaneKind::Properties => "Properties".into(),
            PaneKind::Viewport => "Viewport".into(),
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: TileId,
        pane: &mut Pane,
    ) -> UiResponse {
        match pane.kind {
            PaneKind::Console => {
                ui.heading("Console");
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.text_edit_singleline(&mut self.ui_state.console_filter);
                });
            }
            PaneKind::Properties => {
                ui.heading("Properties");
                ui.checkbox(&mut self.ui_state.props_enabled, "Enabled");
            }
            PaneKind::Viewport => {
                // IMPORTANT:
                // We do NOT paint an opaque background here.
                // The Bevy 3D camera will render behind this tab area.
                ui.label("Viewport");
                ui.label("3D world below");
                ui.allocate_space(ui.available_size());
            }
        }

        UiResponse::None
    }
}

fn setup_egui_camera(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    // Same pattern as your working app:
    // disable auto context, then spawn the primary egui camera manually.
    egui_global_settings.auto_create_primary_context = false;

    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            order: 1,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        RenderLayers::layer(1),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.20, 0.22),
            ..default()
        })),
        RenderLayers::layer(0),
    ));

    // Sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.8).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.55, 0.95),
            ..default()
        })),
        Transform::from_xyz(-1.25, 0.8, 0.0),
        RenderLayers::layer(0),
    ));

    // "Square" -> cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.65, 0.20),
            ..default()
        })),
        Transform::from_xyz(1.25, 0.5, 0.0),
        RenderLayers::layer(0),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(0),
    ));

    // 3D camera that will render ONLY into the egui_tiles viewport tab rect
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            is_active: false, // enabled only when viewport tab is visible
            ..default()
        },
        Transform::from_xyz(4.0, 3.0, 6.0).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
        RenderLayers::layer(0),
        GameViewportCamera,
    ));
}

fn setup_dock(mut commands: Commands) {
    let mut tiles = Tiles::<Pane>::default();

    let console = tiles.insert_pane(Pane {
        kind: PaneKind::Console,
    });
    let properties = tiles.insert_pane(Pane {
        kind: PaneKind::Properties,
    });
    let viewport = tiles.insert_pane(Pane {
        kind: PaneKind::Viewport,
    });

    let root = tiles.insert_tab_tile(vec![console, properties, viewport]);

    commands.insert_resource(DockTree {
        tree: Tree::new("my_tree", root, tiles),
        viewport_tile: viewport,
    });

    commands.insert_resource(UiState {
        console_filter: "hello".to_owned(),
        props_enabled: true,
    });

    commands.insert_resource(ViewportState::default());
}

fn dock_ui_system(
    mut contexts: EguiContexts,
    mut dock: ResMut<DockTree>,
    mut ui_state: ResMut<UiState>,
    mut viewport_state: ResMut<ViewportState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Transparent central panel so the 3D camera can show through in the viewport tab.
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let mut behavior = TreeBehavior {
                ui_state: &mut ui_state,
            };
            dock.tree.ui(&mut behavior, ui);
        });

    // IMPORTANT:
    // Tiles::rect(...) is only valid AFTER tree.ui(...) runs this frame.
    // It returns None if the tile is inactive / hidden.
    viewport_state.rect_points = dock.tree.tiles.rect(dock.viewport_tile);
    viewport_state.pixels_per_point = ctx.pixels_per_point();
}

fn sync_viewport_camera(
    viewport_state: Res<ViewportState>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut q: Query<(&mut Camera, &mut Projection), With<GameViewportCamera>>,
) {
    let Ok((mut camera, mut projection)) = q.single_mut() else {
        return;
    };

    let Some(rect) = viewport_state.rect_points else {
        camera.is_active = false;
        camera.viewport = None;
        return;
    };

    let scale_factor = window.scale_factor();
    let win_width = window.physical_width();
    let win_height = window.physical_height();

    let phys_x = (rect.min.x * scale_factor).max(0.0) as u32;
    let phys_y = (rect.min.y * scale_factor).max(0.0) as u32;
    let mut phys_w = (rect.width() * scale_factor).max(0.0) as u32;
    let mut phys_h = (rect.height() * scale_factor).max(0.0) as u32;

    if phys_x + phys_w > win_width {
        phys_w = win_width.saturating_sub(phys_x);
    }
    if phys_y + phys_h > win_height {
        phys_h = win_height.saturating_sub(phys_y);
    }

    if phys_w == 0 || phys_h == 0 {
        camera.is_active = false;
        camera.viewport = None;
        return;
    }

    camera.is_active = true;
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(phys_x, phys_y),
        physical_size: UVec2::new(phys_w, phys_h),
        depth: 0.0..1.0,
    });

    if let Projection::Perspective(p) = &mut *projection {
        p.aspect_ratio = phys_w as f32 / phys_h as f32;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(
            Startup,
            (setup_egui_camera, setup_scene).before(EguiStartupSet::InitContexts),
        )
        .add_systems(
            Startup,
            (setup_dock).before(EguiStartupSet::InitContexts),
        )
        .add_systems(EguiPrimaryContextPass, dock_ui_system)
        .add_systems(PostUpdate, sync_viewport_camera)
        .run();
}