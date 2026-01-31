use bevy::{
    prelude::*,
    pbr::{Material, MaterialPlugin, MeshMaterial3d},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    light::{NotShadowCaster, NotShadowReceiver},
    camera::visibility::RenderLayers,
};

#[derive(Component)]
pub struct WorldGrid;

#[derive(ShaderType, Clone, Copy, Debug)]
pub struct GridParams {
    pub minor_spacing: f32,
    pub minor_thickness_px: f32,
    pub axis_thickness_px: f32,
    pub fade_distance: f32,

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
}

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WorldGridMaterial>::default())
            .add_systems(Startup, spawn_world_grid);
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
            fade_distance: 150.0,
            _pad: Vec2::ZERO,
            _pad2: Vec2::ZERO,
        },
        alpha_mode: AlphaMode::Blend,
    });

    // Big plane + shader lines = "infinite" illusion.
    // Grid is anchored to world origin because shader uses in.world_position.
    commands.spawn((
        WorldGrid,
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(mat),
        Transform::from_xyz(0.0, -0.001, 0.0).with_scale(Vec3::splat(50_000.0)),
        RenderLayers::layer(0),
        NotShadowCaster,
        NotShadowReceiver,
    ));
}
