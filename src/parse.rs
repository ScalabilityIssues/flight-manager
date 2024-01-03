use std::collections::BTreeSet;

use prost_types::FieldMask;
use sqlx::types::Uuid;
use tonic::Status;

pub fn parse_id(id: String) -> Result<Uuid, Status> {
    Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("'id'"))
}

pub fn parse_update_paths(update_mask: Option<FieldMask>) -> Result<BTreeSet<String>, Status> {
    let update_paths: BTreeSet<_> = match update_mask {
        Some(FieldMask { paths }) => paths.iter().cloned().collect(),
        None => <_>::default(),
    };

    if update_paths.is_empty() {
        Err(Status::invalid_argument("'update_mask' cannot be empty"))
    } else {
        Ok(update_paths)
    }
}
