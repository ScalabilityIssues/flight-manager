use std::ops::DerefMut;

use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub use proto::airports_server::AirportsServer; // what is this? what is airport_server? and AirportsServer? // try to remove

pub mod proto;
mod queries;

#[derive(Debug)]
pub struct AirportsApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl proto::airports_server::Airports for AirportsApp {
    async fn list_airports(
        &self,
        _request: Request<proto::Empty>,
    ) -> Result<Response<proto::AirportList>, Status> {
        let airports = queries::list_airports(&self.db_pool).await?;

        Ok(Response::new(proto::AirportList { airports }))
    }

    async fn get_airport(
        &self,
        request: Request<proto::IdQuery>,
    ) -> Result<Response<proto::AirportRead>, Status> {
        let id = request.into_inner().try_get_uuid()?;

        let airport = queries::get_airport(&self.db_pool, &id).await?;

        Ok(Response::new(airport))
    }

    async fn create_airport(
        &self,
        request: Request<proto::AirportCreate>,
    ) -> std::result::Result<Response<proto::AirportRead>, Status> {
        let airport = queries::create_airport(&self.db_pool, &request.into_inner()).await?;

        Ok(Response::new(airport))
    }

    async fn delete_airport(
        &self,
        request: Request<proto::IdQuery>,
    ) -> std::result::Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().try_get_uuid()?;

        queries::delete_airport(&self.db_pool, &id).await?;

        Ok(Response::new(proto::Empty {}))
    }

    async fn update_airport(
        &self,
        request: Request<proto::AirportUpdate>,
    ) -> std::result::Result<Response<proto::AirportRead>, Status> {
        let update = request.into_inner();
        let id = update.try_get_uuid()?;

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        if let Some(patch) = update.patch {
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
