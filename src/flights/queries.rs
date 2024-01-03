use sqlx::types::time::OffsetDateTime;
use sqlx::{types::Uuid, PgExecutor};

use crate::db::QueryError;

type Result<T> = std::result::Result<T, crate::db::QueryError>;

pub struct Flight {
    pub id: Uuid,
    pub plane_id: Uuid,
    pub origin_id: Uuid,
    pub destination_id: Uuid,
    pub departure_time: OffsetDateTime,
    pub arrival_time: OffsetDateTime,
    pub departure_gate: String,
    pub arrival_gate: String,
    pub status: String,
}

pub async fn list_flights<'a>(ex: impl PgExecutor<'a>) -> Result<Vec<Flight>> {
    let flights = sqlx::query_as!(Flight, "select * from flights")
        .fetch_all(ex)
        .await?;

    Ok(flights)
}

pub async fn get_flight<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<Flight> {
    let flight = sqlx::query_as!(Flight, "select * from flights where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(flight)
}

pub async fn create_flight<'a>(
    ex: impl PgExecutor<'a>,
    plane_id: Uuid,
    origin_id: Uuid,
    destination_id: Uuid,
    departure_time: OffsetDateTime,
    arrival_time: OffsetDateTime,
    departure_gate: String,
    arrival_gate: String,
    status: String,
) -> Result<Flight> {
    let flight = sqlx::query_as!(
        Flight,
        "insert into flights (id, plane_id, origin_id, destination_id, departure_time, arrival_time, departure_gate, arrival_gate, status) values (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8) returning *",
        plane_id,
        origin_id,
        destination_id,
        departure_time,
        arrival_time,
        departure_gate,
        arrival_gate,
        status
    )
    .fetch_one(ex)
    .await?;

    Ok(flight)
}

pub async fn delete_flight<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<()> {
    let res = sqlx::query!("delete from flights where id = $1", id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_plane_id<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    plane_id: &Uuid,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set plane_id = $1 where id = $2",
        plane_id,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_origin_id<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    origin_id: &Uuid,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set origin_id = $1 where id = $2",
        origin_id,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_destination_id<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    destination_id: &Uuid,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set destination_id = $1 where id = $2",
        destination_id,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

// understand how to handle departure and arrival time in rust
pub async fn update_departure_time<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    departure_time: &OffsetDateTime,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set departure_time = $1 where id = $2",
        departure_time,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_arrival_time<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    arrival_time: &OffsetDateTime,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set arrival_time = $1 where id = $2",
        arrival_time,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_departure_gate<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    departure_gate: &str,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set departure_gate = $1 where id = $2",
        departure_gate,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_arrival_gate<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    arrival_gate: &str,
) -> Result<()> {
    let res = sqlx::query!(
        "update flights set arrival_gate = $1 where id = $2",
        arrival_gate,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_status<'a>(ex: impl PgExecutor<'a>, id: &Uuid, status: &str) -> Result<()> {
    let res = sqlx::query!("update flights set status = $1 where id = $2", status, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}
