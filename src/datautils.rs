use prost_types::Timestamp;
use sqlx::types::Uuid;
use time::{Duration, OffsetDateTime};
use tonic::Status;

pub fn convert_odt_to_timestamp(d: OffsetDateTime) -> Timestamp {
    Timestamp {
        seconds: d.unix_timestamp(),
        nanos: d.nanosecond() as i32,
    }
}

pub fn parse_id(id: &str) -> Result<Uuid, Status> {
    id.parse().map_err(|_| Status::invalid_argument("'id'"))
}

pub fn parse_timestamp(timestamp: Option<Timestamp>) -> Result<OffsetDateTime, Status> {
    let Some(Timestamp { seconds, nanos }) = timestamp else {
        return Err(Status::invalid_argument("'timestamp'"));
    };

    OffsetDateTime::from_unix_timestamp(seconds)
        .map(|t| t + Duration::nanoseconds(nanos as i64))
        .map_err(|_| Status::invalid_argument("'timestamp'"))
}
