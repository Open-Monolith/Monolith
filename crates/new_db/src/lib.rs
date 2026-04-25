use bevy::prelude::*;

use crate::db::MonoDb;

pub mod db;
pub mod repo;

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_db);
    }
}

fn setup_db(world: &mut World) {
    let db = MonoDb::open("monolith.sqlite").expect("failed to open monolith.sqlite");
    world.insert_non_send_resource(db);
}