use bevy::{
    prelude::*,
};
use bevy_egui::EguiContexts;
use bevy_egui::egui::{self, Image, Response, TextureId, Vec2};

use crate::theme::ThemeResource;

pub fn setup_icon_textures(
    mut contexts: EguiContexts,
    mut icon_textures: ResMut<mn_core::icons::IconTextures>,
    asset_server: Res<AssetServer>
) {
    // Add each icon image to egui once and cache the texture ID
    for (icon, path) in mn_core::icons::ICON_PATHS.iter() {
        let texture_id: TextureId = contexts.add_image(
            bevy_egui::EguiTextureHandle::Strong(asset_server.load(*path))
        );
        icon_textures.textures.insert(*icon, texture_id);
    }
}

// pub trait IconUiExt {
//     fn icon_button_tinted(
//         &mut self,
//         texture: TextureId,
//         size: Vec2,
//         theme: &ThemeResource,
//         is_selected: bool,
//     ) -> Response;
// }

// impl<'a> IconUiExt for egui::Ui {
//     fn icon_button_tinted(
//         &mut self,
//         texture: TextureId,
//         size: Vec2,
//         theme: &ThemeResource,
//         is_selected: bool,
//     ) -> Response {
//         let pal = theme.current();
//         let tint = if is_selected { pal.accent } else { pal.button };

//         // build sized + tinted Image widget
//         let img = Image::new((texture, size))
//             .tint(tint);

//         self.add(img)
//     }
// }