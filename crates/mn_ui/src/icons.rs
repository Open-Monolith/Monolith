use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::TextureId;

pub fn setup_icon_textures(
    mut contexts: EguiContexts,
    mut icon_textures: ResMut<mn_core::icons::IconTextures>,
    asset_server: Res<AssetServer>
) {
    for (icon, path) in mn_core::icons::ICON_PATHS.iter() {
        let texture_id: TextureId = contexts.add_image(
            bevy_egui::EguiTextureHandle::Strong(asset_server.load(*path))
        );
        icon_textures.textures.insert(*icon, texture_id);
    }
}