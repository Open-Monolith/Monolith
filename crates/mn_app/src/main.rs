use bevy::{
    camera::{CameraOutputMode, Viewport, visibility::RenderLayers},
    prelude::*,
    render::render_resource::BlendState,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui, EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext,
};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .init_resource::<DockData>()
        .add_systems(Startup, setup_system)
        // 1. Calculate UI layout
        .add_systems(EguiPrimaryContextPass, ui_system)
        // 2. Apply viewport to camera (After UI is done)
        .add_systems(PostUpdate, update_viewport_system)
        .run();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Viewport,
    Inspector,
    Hierarchy,
    Assets,
}

#[derive(Resource)]
struct DockData {
    dock_state: DockState<Tab>,
    // Use Option to track if the tab is actually visible
    viewport_rect: Option<egui::Rect>,
}

impl Default for DockData {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![Tab::Viewport]);
        
        let tree = dock_state.main_surface_mut();
        let [viewport, _inspector] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![Tab::Inspector],
        );
        let [_viewport, _hierarchy] = tree.split_left(
            viewport,
            0.2,
            vec![Tab::Hierarchy],
        );

        Self {
            dock_state,
            viewport_rect: None,
        }
    }
}

struct MyTabViewer<'a> {
    viewport_rect: &'a mut Option<egui::Rect>,
}

impl TabViewer for MyTabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport => "Viewport".into(),
            Tab::Inspector => "Inspector".into(),
            Tab::Hierarchy => "Hierarchy".into(),
            Tab::Assets => "Assets".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport => {
                let rect = ui.available_rect_before_wrap();
                // Reserve space so the layout doesn't collapse
                ui.allocate_rect(rect, egui::Sense::hover());
                
                // Set the rect for this frame
                *self.viewport_rect = Some(rect);
                
                // Draw a placeholder background (optional, but good for debugging)
                // We use transparent so Bevy shows through, or dark grey to hide glitches
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_black_alpha(0), 
                );
            }
            Tab::Inspector => { ui.heading("Inspector"); }
            Tab::Hierarchy => { ui.heading("Hierarchy"); }
            Tab::Assets => { ui.heading("Assets"); }
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, Tab::Viewport)
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut dock_data: ResMut<DockData>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) {
    let ctx = contexts.ctx_mut().unwrap();
    let dock_data = dock_data.into_inner();

    // RESET: Assume tab is hidden at start of frame. 
    // If MyTabViewer::ui runs for Tab::Viewport, it will set this to Some(rect).
    dock_data.viewport_rect = None;

    DockArea::new(&mut dock_data.dock_state)
        .show(ctx, 
            &mut MyTabViewer {
                viewport_rect: &mut dock_data.viewport_rect,
            });
}

fn update_viewport_system(
    dock_data: Res<DockData>,
    window: Single<&Window, With<PrimaryWindow>>,
    // Query specifically for the World Camera (not the UI camera)
    mut camera: Single<&mut Camera, (Without<PrimaryEguiContext>, With<Camera2d>)>,
) {
    if let Some(rect) = dock_data.viewport_rect {
        let scale_factor = window.scale_factor();
        let viewport_pos = rect.left_top().to_vec2() * scale_factor;
        let viewport_size = rect.size() * scale_factor;

        let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
        let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

        if physical_size.x > 0 && physical_size.y > 0 {
            camera.viewport = Some(Viewport {
                physical_position,
                physical_size,
                ..default()
            });
            // CRITICAL: Enable camera only when we have a valid target
            camera.is_active = true; 
            return;
        }
    }

    // CRITICAL: Disable camera if tab is hidden or invalid. 
    // This prevents it from defaulting to "Fullscreen" and drawing behind the UI.
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