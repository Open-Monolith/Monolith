use std::collections::BTreeMap;
use strum_macros::{AsRefStr, Display};

// The id of each element
// Later will be revised to use timestampt bit + node + seq
pub type ElementId = i64;

// Element kind. Will be expanded later
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, AsRefStr)]
pub enum ElementKind {
    Wall,
    Floor,
    Roof,
    Device,
    Duct,
    Pipe,
    Level,
    Grid,
    View,
}

// Parameters implementation.
// I hope its not a pain in the ass to revise when duckdb is attached
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParamKey(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum ParamValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    ElemenentRef(ElementId)
}

pub type ElementParams = BTreeMap<ParamKey, ParamValue>;

// Element base class
#[derive(Clone, Debug)]
pub struct ElementHeader {
    pub id: ElementId,
    pub kind: ElementKind,
    pub name: Option<String>,
    pub type_id: Option<ElementId>,
    pub level_id: Option<ElementId>,
    pub params: ElementParams
}

