use sqlx::{types::Uuid, PgConnection};

use crate::db::DatabaseError;

type Result<T> = std::result::Result<T, crate::db::DatabaseError>;

pub struct Airport {
    pub id: Uuid,
    pub icao: String,
    pub iata: String,
    pub name: String,
    pub country: String,
    pub city: String,
    pub deleted: bool,
}

pub async fn list_airports(ex: &mut PgConnection) -> Result<Vec<Airport>> {
    let airports = sqlx::query_as!(Airport, "select * from airports where not deleted")
        .fetch_all(ex)
        .await?;

    Ok(airports)
}

pub async fn list_airports_with_deleted(ex: &mut PgConnection) -> Result<Vec<Airport>> {
    let airports = sqlx::query_as!(Airport, "select * from airports")
        .fetch_all(ex)
        .await?;

    Ok(airports)
}

pub async fn get_airport(ex: &mut PgConnection, id: &Uuid) -> Result<Airport> {
    let airport = sqlx::query_as!(Airport, "select * from airports where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(airport)
}

pub async fn create_airport(
    ex: &mut PgConnection,
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

pub async fn delete_airport(ex: &mut PgConnection, id: &Uuid) -> Result<()> {
    let res = sqlx::query!("update airports set deleted = true where id = $1", id)
        .execute(ex)
        .await?;

    DatabaseError::ensure_single_affected(res)
}
