use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin, MeshMaterial3d},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    window::PrimaryWindow,
};

use crate::camera_controls::TabViewportCamera;
use mn_core::DockData;

#[derive(Component)]
pub struct WorldGrid;

#[derive(Component, Clone, Copy, Debug)]
pub struct WorldGridFollow {
    pub y: f32,      // plane height (avoid z-fighting)
    pub snap: f32,   // snap plane movement to this (usually = minor_spacing)
    pub margin: f32, // scale multiplier relative to camera far (<= 1.4 is safe)
}

/// Uniforms sent to the shader.
#[derive(ShaderType, Clone, Copy, Debug)]
pub struct GridParams {
    pub minor_spacing: f32,      // world units (e.g. 1.0)
    pub minor_thickness_px: f32, // pixels (e.g. 1.0)
    pub axis_thickness_px: f32,  // pixels (e.g. 2.0)
    pub fade_distance: f32,      // world units (fade from CAMERA, not origin)
    // pad to 16-byte multiple (std140-ish expectations)
    pub _pad: Vec2,
    pub _pad2: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
#[uniform(0, GridParams, binding_array(10))]
#[bindless(limit(64))]
pub struct WorldGridMaterial {
    pub params: GridParams,
    pub alpha_mode: AlphaMode,
}

impl From<&WorldGridMaterial> for GridParams {
    fn from(mat: &WorldGridMaterial) -> Self {
        mat.params
    }
}

impl Material for WorldGridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/world_grid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // ✅ Option 2: disable backface culling in the actual render pipeline.
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WorldGridMaterial>::default())
            .add_systems(Startup, spawn_world_grid)
            .add_systems(Update, follow_active_viewport_camera);
    }
}

fn spawn_world_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WorldGridMaterial>>,
) {
    let mat = materials.add(WorldGridMaterial {
        params: GridParams {
            minor_spacing: 1.0,
            minor_thickness_px: 0.4,
            axis_thickness_px: 0.7,
            fade_distance: 1000.0, // fades from camera, so OK anywhere
            _pad: Vec2::ZERO,
            _pad2: Vec2::ZERO,
        },
        alpha_mode: AlphaMode::Blend,
    });

    commands.spawn((
        WorldGrid,
        WorldGridFollow {
            y: -0.001,
            snap: 1.0,
            margin: 1.30, // <= 1.414 keeps corners inside far plane
        },
        Mesh3d(meshes.add(Plane3d::default())), // width/height = 1.0
        MeshMaterial3d(mat),
        Transform::from_xyz(0.0, -0.001, 0.0).with_scale(Vec3::splat(1000.0)),
        GlobalTransform::default(),
    ));
}

fn follow_active_viewport_camera(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,

    cameras: Query<
        (&TabViewportCamera, &Transform, &Projection, &Camera),
        (With<Camera3d>, Without<WorldGrid>), // <- disjoint from grid
    >,

    mut grid_q: Query<
        (&WorldGridFollow, &mut Transform),
        (With<WorldGrid>, Without<Camera3d>), // <- disjoint from cameras
    >,
) {
    let Ok((follow, mut grid_tf)) = grid_q.single_mut() else {
        return;
    };

    // Cursor is in logical coords, same space as DockData viewports
    let cursor = match window.cursor_position() {
        Some(p) => p,
        None => return, // no cursor = don't move the grid this frame
    };

    let Some(active_tab_id) = find_tab_under_cursor(&dock_data, cursor) else {
        return;
    };

    // Find the camera for that tab
    let mut cam_pos: Option<Vec3> = None;
    let mut cam_far: Option<f32> = None;

    for (tag, cam_tf, proj, cam) in cameras.iter() {
        if tag.tab_id != active_tab_id || !cam.is_active {
            continue;
        }
        cam_pos = Some(cam_tf.translation);
        cam_far = Some(projection_far(proj));
        break;
    }

    let (Some(cam_pos), Some(far)) = (cam_pos, cam_far) else {
        return;
    };

    // Snap plane position so it doesn’t “swim” under the camera.
    let snap = follow.snap.max(0.0001);
    let x = (cam_pos.x / snap).round() * snap;
    let z = (cam_pos.z / snap).round() * snap;

    grid_tf.translation = Vec3::new(x, follow.y, z);

    // Critical: scale plane to camera far so the corners stay inside the far clip.
    // Plane3d default has size 1.0 with half-extents 0.5, so scale controls world size.
    let safe_scale = (far * follow.margin).max(10.0);
    grid_tf.scale = Vec3::splat(safe_scale);
}

fn projection_far(proj: &Projection) -> f32 {
    match proj {
        Projection::Perspective(p) => p.far,
        Projection::Orthographic(o) => o.far,
        // if Bevy adds more variants later, fall back safely
        _ => 10_000.0,
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
