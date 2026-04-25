use std::collections::BTreeMap;

use crate::elements::element_kind::ElementKind;

// The id of each element
// Later will be revised to use timestampt bit + node + seq
pub type ElementId = i64;


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

