use sqlx::{types::Uuid, PgExecutor};

use crate::db::QueryError;

type Result<T> = std::result::Result<T, crate::db::QueryError>;

pub struct Airport {
    pub id: Uuid,
    pub icao: String,
    pub iata: String,
    pub name: String,
    pub country: String,
    pub city: String,
}

pub async fn list_airports<'a>(ex: impl PgExecutor<'a>) -> Result<Vec<Airport>> {
    let airports = sqlx::query_as!(Airport, "select * from airports")
        .fetch_all(ex)
        .await?;

    Ok(airports)
}

pub async fn get_airport<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<Airport> {
    let airport = sqlx::query_as!(Airport, "select * from airports where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(airport)
}

pub async fn create_airport<'a>(
    ex: impl PgExecutor<'a>,
    icao: String,
    iata: String,
    name: String,
    country: String,
    city: String,
) -> Result<Airport> {
    let airport = sqlx::query_as!(
        Airport,
        "insert into airports (id, icao, iata, name, country, city) values (gen_random_uuid(), $1, $2, $3, $4, $5) returning *",
        icao,
        iata,
        name,
        country,
        city
    )
    .fetch_one(ex)
    .await?;

    Ok(airport)
}

pub async fn delete_airport<'a>(ex: impl PgExecutor<'a>, id: &Uuid) -> Result<()> {
    let res = sqlx::query!("delete from airports where id = $1", id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_icao<'a>(ex: impl PgExecutor<'a>, id: &Uuid, icao: &str) -> Result<()> {
    let res = sqlx::query!("update airports set icao = $1 where id = $2", icao, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_iata<'a>(ex: impl PgExecutor<'a>, id: &Uuid, iata: &str) -> Result<()> {
    let res = sqlx::query!("update airports set iata = $1 where id = $2", iata, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_name<'a>(ex: impl PgExecutor<'a>, id: &Uuid, name: &str) -> Result<()> {
    let res = sqlx::query!("update airports set name = $1 where id = $2", name, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_country<'a>(ex: impl PgExecutor<'a>, id: &Uuid, country: &str) -> Result<()> {
    let res = sqlx::query!(
        "update airports set country = $1 where id = $2",
        country,
        id
    )
    .execute(ex)
    .await?;

    QueryError::ensure_single_affected(res)
}

pub async fn update_city<'a>(ex: impl PgExecutor<'a>, id: &Uuid, city: &str) -> Result<()> {
    let res = sqlx::query!("update airports set city = $1 where id = $2", city, id)
        .execute(ex)
        .await?;

    QueryError::ensure_single_affected(res)
}
