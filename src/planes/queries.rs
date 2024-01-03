use sqlx::{types::Uuid, PgExecutor};

use crate::db::QueryError;

type Result<T> = std::result::Result<T, crate::db::QueryError>;

pub struct Plane {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub cabin_capacity: i32,
    pub cargo_capacity_kg: i32,
}

pub async fn list_planes<'a>(ex: impl PgExecutor<'a>) -> Result<Vec<Plane>> {
    let planes = sqlx::query_as!(Plane, "select * from planes")
        .fetch_all(ex)
        .await?;

    Ok(planes)
}

pub async fn get_plane<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<Plane> {
    let plane = sqlx::query_as!(Plane, "select * from planes where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(plane)
}

pub async fn create_plane<'a>(
    ex: impl PgExecutor<'a>,
    name: String,
    model: String,
    cabin_cap: i32,
    cargo_cap_kg: i32,
) -> Result<Plane> {
    let plane = sqlx::query_as!(
        Plane,
        "insert into planes (id, name, model, cabin_capacity, cargo_capacity_kg) values (gen_random_uuid(), $1, $2, $3, $4) returning *",
        name,
        model,
        cabin_cap,
        cargo_cap_kg
    )
    .fetch_one(ex)
    .await?;

    Ok(plane)
}

pub async fn delete_plane<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<()> {
    let res = sqlx::query!("delete from planes where id = $1", id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_name<'a>(ex: impl PgExecutor<'a>, id: &Uuid, name: &str) -> Result<()> {
    let res = sqlx::query!("update planes set name = $1 where id = $2", name, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_model<'a>(ex: impl PgExecutor<'a>, id: &Uuid, model: &str) -> Result<()> {
    let res = sqlx::query!("update planes set model = $1 where id = $2", model, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_cabin_cap<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    cabin_capacity: i32,
) -> Result<()> {
    let res = sqlx::query!(
        "update planes set cabin_capacity = $1 where id = $2",
        cabin_capacity,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_cargo_cap<'a>(
    ex: impl PgExecutor<'a>,
    id: &Uuid,
    cargo_capacity: i32,
) -> Result<()> {
    let res = sqlx::query!(
        "update planes set cargo_capacity_kg = $1 where id = $2",
        cargo_capacity,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}
