use std::ops::DerefMut;

use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    parse::{parse_id, parse_update_paths},
    proto::flightmngr::{
        airports_server::Airports, Airport, AirportList, AirportQuery, AirportUpdate,
    },
};

mod queries;

#[derive(Debug)]
pub struct AirportsApp {
    db_pool: PgPool,
}

impl From<queries::Airport> for Airport {
    fn from(airport: queries::Airport) -> Self {
        Self {
            id: airport.id.to_string(),
            icao: airport.icao,
            iata: airport.iata,
            name: airport.name,
            country: airport.country,
            city: airport.city,
        }
    }
}

#[tonic::async_trait]
impl Airports for AirportsApp {
    async fn list_airports(&self, _request: Request<()>) -> Result<Response<AirportList>, Status> {
        let airports = queries::list_airports(&self.db_pool).await?;
        let airports = airports.into_iter().map(Into::into).collect();

        Ok(Response::new(AirportList { airports }))
    }

    async fn get_airport(
        &self,
        request: Request<AirportQuery>,
    ) -> Result<Response<Airport>, Status> {
        let AirportQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        let airport = queries::get_airport(&self.db_pool, &id).await?.into();

        Ok(Response::new(airport))
    }

    async fn create_airport(
        &self,
        request: Request<Airport>,
    ) -> std::result::Result<Response<Airport>, Status> {
        let Airport {
            id: _,
            icao,
            iata,
            name,
            country,
            city,
        } = request.into_inner();

        let airport = queries::create_airport(&self.db_pool, icao, iata, name, country, city)
            .await?
            .into();

        Ok(Response::new(airport))
    }

    async fn delete_airport(
        &self,
        request: Request<AirportQuery>,
    ) -> std::result::Result<Response<()>, Status> {
        let AirportQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        queries::delete_airport(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }

    async fn update_airport(
        &self,
        request: Request<AirportUpdate>,
    ) -> std::result::Result<Response<Airport>, Status> {
        let AirportUpdate {
            id,
            update,
            update_mask,
        } = request.into_inner();
        let id = parse_id(&id)?;
        let update_paths = parse_update_paths(update_mask)?;
        let update = update.unwrap_or_default();

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        for path in update_paths {
            match path.as_str() {
                "icao" => queries::update_icao(t.deref_mut(), &id, &update.icao).await?,
                "iata" => queries::update_iata(t.deref_mut(), &id, &update.iata).await?,
                "name" => queries::update_name(t.deref_mut(), &id, &update.name).await?,
                "country" => queries::update_country(t.deref_mut(), &id, &update.country).await?,
                "city" => queries::update_city(t.deref_mut(), &id, &update.city).await?,
                _ => {
                    return Err(Status::invalid_argument(format!(
                        "'update_mask' contains invalid path '{}'",
                        path
                    )))
                }
            }
        }

        let airport = queries::get_airport(t.deref_mut(), &id).await?.into();

        t.commit()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        Ok(Response::new(airport))
    }
}

impl AirportsApp {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
