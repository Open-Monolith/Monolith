use strum_macros::{EnumIter, Display};


#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LOD {
    #[strum(to_string="LOD 100")]
    Lod100,
    #[strum(to_string="LOD 200")]
    Lod200,
    #[strum(to_string="LOD 300")]
    Lod300,
    #[strum(to_string="LOD 400")]
    Lod400,
    #[strum(to_string="LOD 500")]
    Lod500,
}

#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CutBehavior {
    #[strum(to_string="Show Top")]
    ShowTop,
    #[strum(to_string="Ghost Cut")]
    GhostCut,
    #[strum(to_string="Uncut")]
    Uncut,
}

#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PhasingTemp {
    #[strum(to_string="Existing")]
    Existing,
    #[strum(to_string="Demolition")]
    Demolition,
    #[strum(to_string="New Construction")]
    NewConstruction,
    #[strum(to_string="Furnishing")]
    Furnishing,
    #[strum(to_string="Hand Over")]
    HandOver,
}

#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AnchorStrategy {
    #[strum(to_string="Float")]
    Float,
    #[strum(to_string="Raycast")]
    RayCast,
    #[strum(to_string="Embedded")]
    Embedded,
    #[strum(to_string="Gravity")]
    Gravity,
    #[strum(to_string="Manifold")]
    Manifold,
}


#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum RenderMode {
    #[strum(to_string="Automatic")]
    Automatic,
    #[strum(to_string="Lod 1")]
    Lod1,
    #[strum(to_string="Lod 2")]
    Lod2,
    #[strum(to_string="Lod 3")]
    Lod3,
    #[strum(to_string="No Change")]
    NoChange,
}
