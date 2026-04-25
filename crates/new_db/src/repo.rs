use rusqlite::{params, Result};

use new_core::element::{
    ElementHeader,
    ElementId,
    ElementKind,
    ParamValue
};

use new_core::placement::{
    Placement,
    Curve3,
    Profile3
};

use super::db::MonoDb;

pub fn insert_element(
    db: &MonoDb,
    header: &ElementHeader,
    placement: &Placement,
) {
    db.conn.execute(
        "INSERT INTO elements(kind, name, type_id"
    )?;
}
