use bevy::{camera::visibility::RenderLayers, light::NotShadowCaster, prelude::*, window::PrimaryWindow};
use bevy::transform::TransformSystems;
use mn_core::{
    commands::{ActiveTool, TwoClickRectState},
    tool::ToolId,
    element::ElementId,
};
use uuid::Uuid;

use crate::camera_controls::TabViewportCamera;
use crate::selection::Selectable;

pub const RECT_WIDTH: f32 = 0.20;
pub const RECT_HEIGHT: f32 = 4.0;

#[derive(Component)]
pub struct WallGhost;

#[derive(Resource)]
pub struct WallAssets {
    pub ghost_mesh: Handle<Mesh>,
    pub ghost_mat: Handle<StandardMaterial>,
    pub wall_mat: Handle<StandardMaterial>,
}

impl FromWorld for WallAssets {
    fn from_world(world: &mut World) -> Self {
        let ghost_mesh = {
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            meshes.add(Cuboid::new(1.0, 1.0, 1.0))
        };

        let ghost_mat = {
            let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
            mats.add(StandardMaterial {
                base_color: Color::srgba(0.6, 0.8, 1.0, 0.35),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })
        };

        let wall_mat = {
            let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
            mats.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.9),
                ..default()
            })
        };

        Self { ghost_mesh, ghost_mat, wall_mat }
    }
}

#[derive(Resource, Default)]
pub struct WallGhostEntity(pub Option<Entity>);

pub struct ArchitectWallPlugin;

impl Plugin for ArchitectWallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WallAssets>()
            .init_resource::<WallGhostEntity>()
            .add_systems(
                PostUpdate,
                (wall_ghost_follow_cursor, two_click_wall_system)
                    .after(TransformSystems::Propagate)
                    .chain(),
            );
    }
}

/// Call this when tool activates to guarantee ghost exists.
pub fn ensure_wall_ghost(commands: &mut Commands, assets: &WallAssets, ghost_ent: &mut WallGhostEntity) {
    if ghost_ent.0.is_some() {
        return;
    }

    let e = commands
        .spawn((
            Name::new("WallGhost"),
            WallGhost,
            Mesh3d(assets.ghost_mesh.clone()),
            MeshMaterial3d(assets.ghost_mat.clone()),
            Transform::from_xyz(0.0, RECT_HEIGHT * 0.5, 0.0),
            RenderLayers::layer(0),
            NotShadowCaster,
        ))
        .id();

    ghost_ent.0 = Some(e);
}

/// Updates ghost transform to follow cursor. Guaranteed to exist when tool is active.
fn wall_ghost_follow_cursor(
    window: Single<&Window, With<PrimaryWindow>>,
    active: Res<ActiveTool>,
    state: Res<TwoClickRectState>,
    cameras: Query<(&TabViewportCamera, &Camera, &GlobalTransform), With<Camera3d>>,

    assets: Res<WallAssets>,
    mut ghost_ent: ResMut<WallGhostEntity>,
    mut ghost_tf: Query<&mut Transform, With<WallGhost>>,
    mut commands: Commands,
) {
    if active.tool != Some(ToolId::ArchitectWall) {
        // tool off: remove ghost
        if let Some(e) = ghost_ent.0.take() {
            commands.entity(e).despawn();
        }
        return;
    }

    // guarantee ghost exists even if cursor/raycast fails
    ensure_wall_ghost(&mut commands, &assets, &mut ghost_ent);

    let Some(e) = ghost_ent.0 else { return; };

    let Some(cursor) = window.cursor_position() else { return; };

    // pick the active camera for the active tab (if any); if no tab, just use any active camera
    let cam_pick = if let Some(tab_id) = active.tab_id {
        cameras
            .iter()
            .find(|(tag, cam, _)| tag.tab_id == tab_id && cam.is_active)
            .map(|(_, cam, gt)| (cam, gt))
    } else {
        cameras
            .iter()
            .find(|(_, cam, _)| cam.is_active)
            .map(|(_, cam, gt)| (cam, gt))
    };

    let Some((camera, cam_gt)) = cam_pick else { return; };

    let Ok(ray) = camera.viewport_to_world(cam_gt, cursor) else { return; };

    let Some(mut hit) =
        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    else { return; };
    hit.y = 0.0;

    // before first click: stub preview at cursor
    let (a, b) = match state.start {
        Some(start) => (start, hit),
        None => (hit, hit + Vec3::X * 0.5),
    };

    let Some(t) = ghost_transform(a, b) else { return; };

    if let Ok(mut tf) = ghost_tf.get_mut(e) {
        *tf = t;
    } else {
        // ghost entity got deleted unexpectedly; respawn next frame
        ghost_ent.0 = None;
    }
}

#[inline]
fn ghost_transform(start: Vec3, end: Vec3) -> Option<Transform> {
    let mut dir = end - start;
    dir.y = 0.0;
    let len = dir.length();
    if len < 0.001 {
        return None;
    }

    let mid = (start + end) * 0.5;
    let rot = Quat::from_rotation_arc(Vec3::X, dir / len);

    // ghost scales unit mesh (preview only; not your final wall)
    Some(
        Transform::from_translation(Vec3::new(mid.x, RECT_HEIGHT * 0.5, mid.z))
            .with_rotation(rot)
            .with_scale(Vec3::new(len, RECT_HEIGHT, RECT_WIDTH)),
    )
}

/// Second click: spawn a NEW mesh wall, kill ghost, disable tool.
fn two_click_wall_system(
    window: Single<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,

    mut active: ResMut<ActiveTool>,
    mut state: ResMut<TwoClickRectState>,
    cameras: Query<(&TabViewportCamera, &Camera, &GlobalTransform), With<Camera3d>>,

    assets: Res<WallAssets>,
    mut ghost_ent: ResMut<WallGhostEntity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if active.tool != Some(ToolId::ArchitectWall) {
        return;
    }
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else { return; };

    let cam_pick = if let Some(tab_id) = active.tab_id {
        cameras
            .iter()
            .find(|(tag, cam, _)| tag.tab_id == tab_id && cam.is_active)
            .map(|(_, cam, gt)| (cam, gt))
    } else {
        cameras
            .iter()
            .find(|(_, cam, _)| cam.is_active)
            .map(|(_, cam, gt)| (cam, gt))
    };

    let Some((camera, cam_gt)) = cam_pick else { return; };

    let Ok(ray) = camera.viewport_to_world(cam_gt, cursor) else { return; };

    let Some(mut hit) =
        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    else { return; };
    hit.y = 0.0;

    if state.start.is_none() {
        state.start = Some(hit);
        return;
    }

    let start = state.start.take().unwrap();
    let end = hit;

    // ✅ NEW wall mesh per wall (your request)
    let mut dir = end - start;
    dir.y = 0.0;
    let len = dir.length();
    if len >= 0.001 {
        let mid = (start + end) * 0.5;
        let rot = Quat::from_rotation_arc(Vec3::X, dir / len);

        let wall_mesh = meshes.add(Cuboid::new(len, RECT_HEIGHT, RECT_WIDTH));

        commands.spawn((
            ElementId(Uuid::new_v4()),
            Name::new("Wall"),
            Mesh3d(wall_mesh),
            MeshMaterial3d(assets.wall_mat.clone()),
            Transform::from_translation(Vec3::new(mid.x, RECT_HEIGHT * 0.5, mid.z)).with_rotation(rot),
            RenderLayers::layer(0),
            Selectable,
        ));
    }

    // disable tool + cleanup
    active.tool = None;
    active.tab_id = None;
    state.start = None;

    if let Some(e) = ghost_ent.0.take() {
        commands.entity(e).despawn();
    }
}
