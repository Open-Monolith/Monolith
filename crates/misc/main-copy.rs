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


pub fn hex_to_color(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
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


    egui::TopBottomPanel::top("main_menu_bar")
        .show_separator_line(false)
        .frame(egui::Frame {
            inner_margin: egui::Margin::symmetric(15, 8),
            fill: hex_to_color("#181818"),
            stroke: egui::Stroke::NONE,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                file_menu(ctx, ui);
                edit_menu(ctx, ui);
                window_menu(ctx, ui);
                view_menu(ctx, ui);
            });
        });

    DockArea::new(&mut dock_data.dock_state)
        .show(ctx, 
            &mut MyTabViewer {
                viewport_rect: &mut dock_data.viewport_rect,
            });
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

    if let Some(rect) = dock_data.viewport_rect {
        let scale_factor = window.scale_factor();
        
        // 2. Convert Egui (Logical) -> Physical Pixels
        let viewport_pos = rect.left_top().to_vec2() * scale_factor;
        let viewport_size = rect.size() * scale_factor;

        // 3. Cast to u32
        let mut phys_x = viewport_pos.x.max(0.0) as u32;
        let mut phys_y = viewport_pos.y.max(0.0) as u32;
        let mut phys_w = viewport_size.x.max(0.0) as u32;
        let mut phys_h = viewport_size.y.max(0.0) as u32;

        // 4. CRITICAL FIX: Clamp the viewport to the Window Size
        // If the viewport tries to go outside the window (because of resizing lag), chop it off.
        if phys_x + phys_w > win_width {
            phys_w = win_width.saturating_sub(phys_x);
        }
        if phys_y + phys_h > win_height {
            phys_h = win_height.saturating_sub(phys_y);
        }

        // 5. Only apply if we still have a valid size
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



fn file_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("New Project").clicked() {}
        if ui.button("Open Project...").clicked() {}
        ui.menu_button("Open Recent", |ui| {
            if ui.button("Kahuina.monolith").clicked() {}
            if ui.button("Uptown_mall.monolith").clicked() {}
            if ui.button("Launiu Project.monolith").clicked() {}
        });
        if ui.button("Save").clicked() {}
        if ui.button("Save as...").clicked() {}
        ui.separator();
        ui.menu_button("Import", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("glTF / GLB").clicked() {}
            if ui.button("CAD (.dxf)").clicked() {}
        });
        ui.menu_button("Export", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("Sheet (.pdf)").clicked() {}
        });
        ui.separator();
        if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

fn edit_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Edit", |ui| {
        if ui.button("Undo (Ctrl + Z)").clicked() {}
        if ui.button("Redo (Ctrl + Y)").clicked() {}
        ui.separator();
        if ui.button("Cut (Ctrl + X)").clicked() {}
        if ui.button("Copy (Ctrl + C)").clicked() {}
        if ui.button("Paste (Ctrl + V)").clicked() {}
        if ui.button("Delete (Del)").clicked() {}
        ui.separator();
        ui.menu_button("Select All...", |ui| {
            if ui.button("in Current View").clicked() {}
            if ui.button("in Entire Project").clicked() {}
        });
        if ui.button("Deselect All (Alt + A)").clicked() {}
        ui.separator();
        if ui.button("Commands (Space)").clicked() {}
        if ui.button("Preferences...").clicked() {}
    });
}

fn window_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Window", |ui| {
        ui.menu_button("Workspaces", |ui| {
            if ui.button("Modeling").clicked() {}
            if ui.button("Drafting").clicked() {}
            if ui.button("Forging").clicked() {}
            if ui.button("Rendering").clicked() {}
            if ui.button("Scripting").clicked() {}
            if ui.button("Collaboration").clicked() {}
        });
        ui.separator();
        if ui.button("Scene View").clicked() {}
        if ui.button("Properties").clicked() {}
        if ui.button("File Explorer").clicked() {}
        if ui.button("Scripting").clicked() {}
        if ui.button("Asset Browser").clicked() {}
        if ui.button("Console").clicked() {}
        ui.separator();
        if ui.button("Toggle Fullscreen (F11)").clicked() {}
        if ui.button("Reset Layout").clicked() {}
    });
}

fn view_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("View", |ui| {
        if ui.button("Documentation").clicked() {}
        if ui.button("Keyboard Shortcuts").clicked() {}
        if ui.button("Report a Bug").clicked() {}
        if ui.button("Community").clicked() {}
        ui.separator();
        if ui.button("Developer Tools").clicked() {}
        if ui.button("About Monolith").clicked() {}
    });
}