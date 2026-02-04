use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use mn_core::{DockData, commands::{ActiveTool, TwoClickRectState}, element::ElementId, tool::ToolId};
use uuid::Uuid;

use crate::{camera_controls::TabViewportCamera, selection::Selectable};


// “Fixed width and height”; length comes from 2 clicked points.
pub const RECT_WIDTH: f32 = 0.20;
pub const RECT_HEIGHT: f32 = 5.;


pub fn two_click_rect_system(
    dock: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,

    mut active: ResMut<ActiveTool>,
    mut state: ResMut<TwoClickRectState>,

    cameras: Query<(&TabViewportCamera, &Camera, &GlobalTransform), With<Camera3d>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Only run when this tool is active
    if active.tool != Some(ToolId::ArchitectWall) {
        return;
    }

    // Only react to “fresh” clicks
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else { return; };
    let Some(tab_under_cursor) = find_tab_under_cursor(&dock, cursor) else { return; };

    // Optional: only allow drawing in the same viewport tab where the tool was selected
    if active.tab_id != Some(tab_under_cursor) {
        return;
    }

    // Find the camera for this viewport tab
    let mut cam: Option<(&Camera, &GlobalTransform)> = None;
    for (tag, camera, cam_gt) in cameras.iter() {
        if tag.tab_id == tab_under_cursor && camera.is_active {
            cam = Some((camera, cam_gt));
            break;
        }
    }
    let Some((camera, cam_gt)) = cam else { return; };

    // Build a ray from cursor -> world
    let Ok(ray) = camera.viewport_to_world(cam_gt, cursor) else { return; }; // Bevy 0.18 API :contentReference[oaicite:1]{index=1}

    // Intersect with the infinite plane at y=0 (XZ plane, Y-up)
    let plane = InfinitePlane3d::new(Vec3::Y); // normal is +Y :contentReference[oaicite:2]{index=2}
    let Some(mut hit) = ray.plane_intersection_point(Vec3::ZERO, plane) else { return; }; // :contentReference[oaicite:3]{index=3}
    hit.y = 0.0; // force vertical coord to zero

    // First click = start point
    if state.start.is_none() {
        state.start = Some(hit);
        return;
    }

    // Second click = end point => spawn mesh
    let start = state.start.take().unwrap();
    let end = hit;

    spawn_rect_between_points(start, end, &mut commands, &mut meshes, &mut materials);

    active.tool = None;
    active.tab_id = None;
    state.start = None; 

}

fn spawn_rect_between_points(
    start: Vec3,
    end: Vec3,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mut dir = end - start;
    dir.y = 0.0;
    let len = dir.length();
    if len < 0.001 {
        return;
    }

    let mid = (start + end) * 0.5;

    // Make the cuboid’s local +X axis point along the segment
    let rot = Quat::from_rotation_arc(Vec3::X, dir.normalize());

    // Cuboid::new(x_len, y_len, z_len) takes FULL lengths in Bevy 0.18 :contentReference[oaicite:4]{index=4}
    let mesh = meshes.add(Cuboid::new(len, RECT_HEIGHT, RECT_WIDTH));
    let mat = materials.add(Color::srgb(0.8, 0.8, 0.9));

    commands.spawn((
        ElementId(Uuid::new_v4()),
        Name::new("TwoClickRect"),
        Transform::from_translation(Vec3::new(mid.x, RECT_HEIGHT * 0.5, mid.z)).with_rotation(rot),
        GlobalTransform::default(),
        Mesh3d(mesh),
        MeshMaterial3d(mat),
        RenderLayers::layer(0),
        Selectable,
    ));
}

fn find_tab_under_cursor(dock_data: &DockData, cursor: Vec2) -> Option<u32> {
    for (&tab_id, &(x, y, w, h)) in dock_data.viewports.iter() {
        if cursor.x >= x && cursor.x <= x + w && cursor.y >= y && cursor.y <= y + h {
            return Some(tab_id);
        }
    }
    None
}