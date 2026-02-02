use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ElementId(pub Uuid);