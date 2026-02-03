use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::render::render_resource::Face;
use bevy::window::PrimaryWindow;

use mn_core::DockData;

use crate::camera_controls::TabViewportCamera;
use crate::camera_controls::BimOrbitCamera;

pub struct SelectionPlugin;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub current: Option<Entity>,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
struct SelectionOutline;

#[derive(Resource, Clone)]
struct OutlineMaterial(pub Handle<StandardMaterial>);

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Startup, setup_outline_material)
            .add_systems(
                Update,
                (
                    select_with_left_click,
                    spawn_outline_for_selected,
                    remove_outline_for_deselected,
                ),
            );
    }
}

fn setup_outline_material(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.0),
        unlit: true,
        cull_mode: Some(Face::Front),
        depth_bias: 1.0,
        ..default()
    });
    commands.insert_resource(OutlineMaterial(mat));
}

fn select_with_left_click(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ray_cast: MeshRayCast,

    cams: Query<(&Camera, &GlobalTransform, &TabViewportCamera, &mut BimOrbitCamera)>,
    selectables: Query<(), With<Selectable>>,
    exists: Query<(), ()>,

    mut sel: ResMut<SelectionState>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = window.cursor_position() else { return; };

    let Some(tab_id) = find_tab_under_cursor(&dock_data, cursor) else {
        set_selection(&mut sel, None, &mut commands, &exists);
        return;
    };

    let mut cam_hit = None;
    let mut bimCam_hit: Option<&mut BimOrbitCamera> = None;
    for (cam, cam_tf, tag, _) in &cams {
        if tag.tab_id == tab_id {
            cam_hit = Some((cam, cam_tf));
            break;
        }
    }
    let Some((camera, cam_tf)) = cam_hit else { return; };

    let Ok(ray) = camera.viewport_to_world(cam_tf, cursor) else {
        set_selection(&mut sel, None, &mut commands, &exists);
        return;
    };

    
    let filter = |e: Entity| selectables.contains(e);
    let settings = MeshRayCastSettings::default()
        .with_filter(&filter)
        .with_visibility(RayCastVisibility::Visible)
        .always_early_exit();

    let hit_entity = ray_cast
        .cast_ray(ray, &settings)
        .first()
        .map(|(e, _hit)| *e);

    set_selection(&mut sel, hit_entity, &mut commands, &exists);
}

fn set_selection(
    sel: &mut SelectionState,
    new: Option<Entity>,
    commands: &mut Commands,
    exists: &Query<(), ()>,
) {
    if sel.current == new { return; }

    if let Some(old) = sel.current.take() {
        if exists.get(old).is_ok() {
            commands.entity(old).remove::<Selected>();
        }
    }

    if let Some(e) = new {
        if exists.get(e).is_ok() {
            commands.entity(e).insert(Selected);
            sel.current = Some(e);
        }
    }
}

fn spawn_outline_for_selected(
    mut commands: Commands,
    outline_mat: Res<OutlineMaterial>,
    q_added: Query<(Entity, &Mesh3d), Added<Selected>>,
) {
    for (e, mesh) in &q_added {
        commands.entity(e).with_children(|c| {
            c.spawn((
                Name::new("SelectionOutline"),
                Mesh3d(mesh.0.clone()),
                MeshMaterial3d(outline_mat.0.clone()),
                Transform::from_scale(Vec3::splat(1.03)),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                SelectionOutline,
            ));
        });
    }
}

fn remove_outline_for_deselected(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    children: Query<&Children>,
    outlines: Query<(), With<SelectionOutline>>,
) {
    for e in removed.read() {
        if let Ok(kids) = children.get(e) {
            for child in kids.iter() {
                if outlines.contains(child) {
                    commands.entity(child).despawn();
                }
            }
        }
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
