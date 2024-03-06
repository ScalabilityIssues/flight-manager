use sqlx::{types::Uuid, PgConnection};

use crate::db::QueryError;

type Result<T> = std::result::Result<T, crate::db::QueryError>;

pub struct Plane {
    pub id: Uuid,
    pub model: String,
    pub cabin_capacity: i32,
    pub cargo_capacity_kg: i32,
    pub deleted: bool,
}

pub async fn list_planes(ex: &mut PgConnection) -> Result<Vec<Plane>> {
    let planes = sqlx::query_as!(Plane, "select * from planes where not deleted")
        .fetch_all(ex)
        .await?;

    Ok(planes)
}

pub async fn list_planes_with_deleted(ex: &mut PgConnection) -> Result<Vec<Plane>> {
    let planes = sqlx::query_as!(Plane, "select * from planes")
        .fetch_all(ex)
        .await?;

    Ok(planes)
}

pub async fn get_plane(ex: &mut PgConnection, id: &Uuid) -> Result<Plane> {
    let plane = sqlx::query_as!(Plane, "select * from planes where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(plane)
}

pub async fn create_plane(
    ex: &mut PgConnection,
    model: String,
    cabin_cap: i32,
    cargo_cap_kg: i32,
) -> Result<Plane> {
    let plane = sqlx::query_as!(
        Plane,
        "insert into planes (id, model, cabin_capacity, cargo_capacity_kg) values (gen_random_uuid(), $1, $2, $3) returning *",
        model,
        cabin_cap,
        cargo_cap_kg
    )
    .fetch_one(ex)
    .await?;

    Ok(plane)
}

pub async fn delete_plane(ex: &mut PgConnection, id: &Uuid) -> Result<()> {
    let res = sqlx::query!("update planes set deleted = true where id = $1", id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}
