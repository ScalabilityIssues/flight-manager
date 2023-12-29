use sqlx::types::Uuid;

tonic::include_proto!("airports");

impl IdQuery {
    pub fn try_get_uuid(&self) -> Result<Uuid, tonic::Status> {
        Uuid::parse_str(&self.id).map_err(|_| tonic::Status::invalid_argument("id"))
    }
}

impl AirportUpdate {
    pub fn try_get_uuid(&self) -> Result<Uuid, tonic::Status> {
        Uuid::parse_str(&self.id).map_err(|_| tonic::Status::invalid_argument("id"))
    }
}
