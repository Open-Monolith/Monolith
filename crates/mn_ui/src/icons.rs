use bevy::{
    prelude::*,
};
use bevy::platform::collections::HashMap;
use mn_core::icons::{ICON_PATHS, Icon, Icons};

pub struct IconsPlugin;

impl Plugin for IconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_icons);
    }
}

fn load_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut map: HashMap<Icon, Handle<Image>> = HashMap::new();
    
    for (icon, path) in ICON_PATHS.iter() {
        map.insert(*icon, asset_server.load(*path));
    }

    commands.insert_resource(Icons { handles: map});

}