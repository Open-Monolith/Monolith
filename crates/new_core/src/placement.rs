#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Quart3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

pub struct Pose3 {
    pub position: Point3,
    pub rotation: Quart3
}

#[derive(Clone, Debug)]
pub enum Placement {
    None,
    Pose3
}