use std::error::Error;

use sqlx::{postgres::PgQueryResult, types::Uuid, PgExecutor};

use crate::airports::proto;

pub enum AirportQueryError {
    NotFound,
    Other(Box<dyn Error + Send + Sync + 'static>),
}

impl From<sqlx::Error> for AirportQueryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AirportQueryError::NotFound,
            _ => AirportQueryError::Other(Box::new(err)),
        }
    }
}

impl From<AirportQueryError> for tonic::Status {
    fn from(err: AirportQueryError) -> Self {
        match err {
            AirportQueryError::NotFound => tonic::Status::not_found("airport"),
            AirportQueryError::Other(err) => tonic::Status::from_error(err),
        }
    }
}

fn check_affected(res: PgQueryResult) -> Result<(), AirportQueryError> {
    if res.rows_affected() == 0 {
        Err(AirportQueryError::NotFound)
    } else {
        Ok(())
    }
}

pub async fn list_airports<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
) -> Result<Vec<proto::AirportRead>, AirportQueryError> {
    let airports = sqlx::query_as!(proto::AirportRead, "select * from airports")
        .fetch_all(ex)
        .await?;

    Ok(airports)
}

pub async fn get_airport<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
) -> Result<proto::AirportRead, AirportQueryError> {
    let airport = sqlx::query_as!(
        proto::AirportRead,
        "select * from airports where id = $1",
        id
    )
    .fetch_one(ex)
    .await?;

    Ok(airport)
}

pub async fn create_airport<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    airport: &proto::AirportCreate, // why airport is here and not used inside the function? and rust is not complaining about it?
) -> Result<proto::AirportRead, AirportQueryError> {
    let airport = sqlx::query_as!(
        proto::AirportRead,
        "insert into airports (id, icao, iata, name, country, city) values (gen_random_uuid(), $1, $2, $3, $4, $5) returning *",
        airport.icao,
        airport.iata,
        airport.name,
        airport.country,
        airport.city
    )
    .fetch_one(ex)
    .await?;

    Ok(airport)
}

pub async fn delete_airport<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!("delete from airports where id = $1", id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_icao<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    icao: &str,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!("update airports set icao = $1 where id = $2", icao, id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_iata<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    iata: &str,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!("update airports set iata = $1 where id = $2", iata, id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_name<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    name: &str,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!("update airports set name = $1 where id = $2", name, id)
        .execute(ex)
        .await?;

    check_affected(res)
}

pub async fn update_country<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    country: &str,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!(
        "update airports set country = $1 where id = $2",
        country,
        id
    )
    .execute(ex)
    .await?;

    check_affected(res)
}

pub async fn update_city<'e, 'c: 'e, E: 'e + PgExecutor<'c>>(
    ex: E,
    id: &Uuid,
    city: &str,
) -> Result<(), AirportQueryError> {
    let res = sqlx::query!("update airports set city = $1 where id = $2", city, id)
        .execute(ex)
        .await?;

    check_affected(res)
}
