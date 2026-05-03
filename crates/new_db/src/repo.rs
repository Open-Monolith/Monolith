use rusqlite::{params, Result};

use new_core::element::{ElementHeader, ElementId};
use new_core::placement::{Placement, Pose3};

use super::db::MonoDb;

pub fn insert_element(
    db: &MonoDb,
    header: &ElementHeader,
    placement: &Placement,
) -> Result<ElementId> {
    let kind_type = header
        .kind_type
        .as_ref()
        .map(|kind_type| kind_type.to_string());

    db.conn.execute(
        "
        INSERT INTO elements (name, kind, kind_type, spec_id, level_id)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        params![
            header.name.as_deref(),
            header.kind.to_string(),
            kind_type.as_deref(),
            header.spec_id,
            header.level_id,
        ],
    )?;

    let element_id: ElementId = db.conn.last_insert_rowid();
    insert_placement(db, element_id, placement)?;

    Ok(element_id)
}

pub fn insert_placement(
    db: &MonoDb,
    element_id: ElementId,
    placement: &Placement,
) -> Result<()> {
    match placement {
        Placement::None => Ok(()),
        Placement::Pose(pose) => insert_pose(db, element_id, pose),
        _ => Ok(()), // curve/profile not yet implemented
    }
}

pub fn insert_pose(db: &MonoDb, element_id: ElementId, pose: &Pose3) -> Result<()> {
    db.conn.execute(
        "
        INSERT INTO placement_pose
        (element_id, px, py, pz, qx, qy, qz, qw)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ",
        params![
            element_id,
            pose.position.x,
            pose.position.y,
            pose.position.z,
            pose.rotation.x,
            pose.rotation.y,
            pose.rotation.z,
            pose.rotation.w
        ],
    )?;

    Ok(())
}