#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quat3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pose3 {
    pub position: Point3,
    pub rotation: Quat3
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Curve3 {
    pub points: Vec<Point3>
}

impl Curve3 {
    pub fn line(start: Point3, end: Point3) -> Self {
        Self {
            points: vec![start, end]
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.points.len() >= 2
    }

    pub fn start(&self) -> Option<Point3> {
        self.points.first().copied()
    }

    pub fn end(&self) -> Option<Point3> {
        self.points.last().copied()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Profile3 {
    loops: Vec<Curve3>,}


#[derive(Clone, Debug)]
pub enum Placement {
    None,
    Pose(Pose3),
    Curve(Curve3),
    Profile(Profile3)
}