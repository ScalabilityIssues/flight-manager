use super::queries;
use crate::proto;

impl From<queries::Plane> for proto::flightmngr::Plane {
    fn from(plane: queries::Plane) -> Self {
        Self {
            id: plane.id.to_string(),
            model: plane.model,
            cabin_capacity: plane.cabin_capacity,
            cargo_capacity_kg: plane.cargo_capacity_kg,
            deleted: plane.deleted,
        }
    }
}
