use std::error::Error;

use sqlx::{postgres::PgQueryResult, types::Uuid, PgExecutor};

use crate::proto::flightmngr::{PlaneRead, PlaneCreate};

pub enum PlaneQueryError {
    NotFound,
    Other(Box<dyn Error + Send + Sync + 'static>),
}

impl From<sqlx::Error> for PlaneQueryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => PlaneQueryError::NotFound,
            _ => PlaneQueryError::Other(Box::new(err)),
        }
    }
}

impl From<PlaneQueryError> for tonic::Status {
    fn from(err: PlaneQueryError) -> Self {
        match err {
            PlaneQueryError::NotFound => tonic::Status::not_found("plane"),
            PlaneQueryError::Other(err) => tonic::Status::from_error(err),
        }
    }
}

fn check_affected(res: PgQueryResult) -> Result<(), PlaneQueryError> {
    if res.rows_affected() == 0 {
        Err(PlaneQueryError::NotFound)
    } else {
        Ok(())
    }
}

pub async fn list_planes<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
) -> Result<Vec<PlaneRead>, PlaneQueryError> {
    let planes = sqlx::query_as!(PlaneRead, "select * from planes")
        .fetch_all(ex)
        .await?;

    Ok(planes)
}

pub async fn get_plane<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
) -> Result<PlaneRead, PlaneQueryError> {
    let plane = sqlx::query_as!(PlaneRead, "select * from planes where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(plane)
}

pub async fn create_plane<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    plane: &PlaneCreate,
) -> Result<PlaneRead, PlaneQueryError> {
    let plane = sqlx::query_as!(
        PlaneRead,
        "insert into planes (id, name, model, cabin_capacity, cargo_capacity_kg) values (gen_random_uuid(), $1, $2, $3, $4) returning *",
        plane.name,
        plane.model,
        plane.cabin_capacity,
        plane.cargo_capacity_kg
    )
    .fetch_one(ex)
    .await?;

    Ok(plane)
}

pub async fn delete_plane<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
) -> Result<(), PlaneQueryError> {
    let res = sqlx::query!("delete from planes where id = $1", id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_name<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    name: &str,
) -> Result<(), PlaneQueryError> {
    let res = sqlx::query!("update planes set name = $1 where id = $2", name, id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_model<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    model: &str,
) -> Result<(), PlaneQueryError> {
    let res = sqlx::query!("update planes set model = $1 where id = $2", model, id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_cabin_cap<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    cabin_capacity: i32,
) -> Result<(), PlaneQueryError> {
    let res = sqlx::query!(
        "update planes set cabin_capacity = $1 where id = $2",
        cabin_capacity,
        id
    )
    .execute(ex)
    .await?;

    check_affected(res)
}

pub async fn update_cargo_cap<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    cargo_capacity: i32,
) -> Result<(), PlaneQueryError> {
    let res = sqlx::query!(
        "update planes set cargo_capacity_kg = $1 where id = $2",
        cargo_capacity,
        id
    )
    .execute(ex)
    .await?;

    check_affected(res)
}
