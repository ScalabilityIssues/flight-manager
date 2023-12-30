use std::ops::DerefMut;

use sqlx::{types::Uuid, PgPool};
use tonic::{Request, Response, Status};

use crate::proto::flightmngr::{
    airports_server::Airports, AirportCreate, AirportList, AirportQuery, AirportRead, AirportUpdate,
};

mod queries;

#[derive(Debug)]
pub struct AirportsApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl Airports for AirportsApp {
    async fn list_airports(&self, _request: Request<()>) -> Result<Response<AirportList>, Status> {
        let airports = queries::list_airports(&self.db_pool).await?;

        Ok(Response::new(AirportList { airports }))
    }

    async fn get_airport(
        &self,
        request: Request<AirportQuery>,
    ) -> Result<Response<AirportRead>, Status> {
        let AirportQuery { id } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        let airport = queries::get_airport(&self.db_pool, &id).await?;

        Ok(Response::new(airport))
    }

    async fn create_airport(
        &self,
        request: Request<AirportCreate>,
    ) -> std::result::Result<Response<AirportRead>, Status> {
        let airport = queries::create_airport(&self.db_pool, &request.into_inner()).await?;

        Ok(Response::new(airport))
    }

    async fn delete_airport(
        &self,
        request: Request<AirportQuery>,
    ) -> std::result::Result<Response<()>, Status> {
        let AirportQuery { id } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        queries::delete_airport(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }

    async fn update_airport(
        &self,
        request: Request<AirportUpdate>,
    ) -> std::result::Result<Response<AirportRead>, Status> {
        let AirportUpdate { id, patch } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        if let Some(patch) = patch {
            if let Some(icao) = patch.icao {
                queries::update_icao(t.deref_mut(), &id, &icao).await?;
            }
            if let Some(iata) = patch.iata {
                queries::update_iata(t.deref_mut(), &id, &iata).await?;
            }
            if let Some(name) = patch.name {
                queries::update_name(t.deref_mut(), &id, &name).await?;
            }
            if let Some(country) = patch.country {
                queries::update_country(t.deref_mut(), &id, &country).await?;
            }
            if let Some(city) = patch.city {
                queries::update_city(t.deref_mut(), &id, &city).await?;
            }
        }

        let airport = queries::get_airport(t.deref_mut(), &id).await?;

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
