use bevy::prelude::*;
use strum::VariantArray;

pub fn setup_icon_textures(
    mut contexts: bevy_egui::EguiContexts,
    mut icon_textures: ResMut<mn_core::icons::IconTextures>,
    asset_server: Res<AssetServer>
) {
    icon_textures.textures.reserve(mn_core::icons::Icon::VARIANTS.len());

    for &icon in mn_core::icons::Icon::VARIANTS {
        if icon_textures.textures.contains_key(&icon) {
            continue;
        }
        let Some(path) = mn_core::icons::icon_path(&icon) else { continue };
        let texture_id: bevy_egui::egui::TextureId = contexts.add_image(
            bevy_egui::EguiTextureHandle::Strong(asset_server.load(path))
        );
        icon_textures.textures.insert(icon, texture_id);
    }
}