use bevy::prelude::*;
use bevy_egui::{
    EguiPostUpdateSet,
    EguiStartupSet
};

use crate::camera::{
    controls::viewport_camera_controls_system,
    setup_egui::setup_egui_camera,
    setup_scene::setup_scene,
    viewport::sync_viewport_cameras
};

pub struct AppCameraPlugin;

impl Plugin for AppCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_egui_camera, setup_scene).before(EguiStartupSet::InitContexts),
        )
        .add_systems(Update, viewport_camera_controls_system)
        .add_systems(PostUpdate, 
            sync_viewport_cameras.after(EguiPostUpdateSet::EndPass),
        );

    }
}