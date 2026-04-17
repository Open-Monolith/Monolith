use bevy::camera::{
    CameraOutputMode, ClearColorConfig, visibility::RenderLayers,
};
use bevy::prelude::*;
use bevy::render::render_resource::BlendState;
use bevy_egui::{
    EguiGlobalSettings, PrimaryEguiContext,
};


pub fn setup_egui_camera(
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
