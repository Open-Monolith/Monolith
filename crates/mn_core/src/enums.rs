#[derive(strum_macros::EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum LOD {
    Lod100,
    Lod200,
    Lod300,
    Lod400,
    Lod500,
}

#[derive(strum_macros::EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum CutBehavior {
    ShowTop,
    GhostCut,
    Uncut,
}

#[derive(strum_macros::EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum PhasingTemp {
    Existing,
    Demolition,
    NewConstruction,
    Furnishing,
    HandOver,
}

#[derive(strum_macros::EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum AnchorStrategy {
    Float,
    RayCast,
    Embedded,
    Gravity,
    Manifold,
}


#[derive(strum_macros::EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum RenderMode {
    Automatic,
    Lod1,
    Lod2,
    Lod3,
    NoChange,
}
