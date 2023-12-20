use sqlx::types::Uuid;

tonic::include_proto!("planes");
pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("planes_descriptor");

impl IdQuery {
    pub fn try_get_uuid(&self) -> Result<Uuid, tonic::Status> {
        Uuid::parse_str(&self.id).map_err(|_| tonic::Status::invalid_argument("id"))
    }
}

impl PlaneUpdate {
    pub fn try_get_uuid(&self) -> Result<Uuid, tonic::Status> {
        Uuid::parse_str(&self.id).map_err(|_| tonic::Status::invalid_argument("id"))
    }
}