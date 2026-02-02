use bevy::prelude::*;

// There are three types of Primitive
// PointBased - asset floats in a point
// LinearBased - asset cxan expand and form a line and junction based on set rules
// PerimeterBased - asset can be drawn with a perimeter e.g. a floor, ceiling
#[derive(Component, Debug, Clone, Copy)]
pub enum PrimitiveKind {
    PointBased,
    LinearBased,
    PerimeterBased
}