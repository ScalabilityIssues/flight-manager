use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    datautils::parse_id,
    proto::flightmngr::{
        airports_server::Airports, Airport, CreateAirportRequest, DeleteAirportRequest,
        GetAirportRequest, ListAirportsRequest, ListAirportsResponse,
    },
};

mod map;
mod queries;

#[derive(Debug)]
pub struct AirportsApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl Airports for AirportsApp {
    async fn list_airports(
        &self,
        request: Request<ListAirportsRequest>,
    ) -> Result<Response<ListAirportsResponse>, Status> {
        let ListAirportsRequest { show_deleted } = request.into_inner();

        let airports = if show_deleted {
            queries::list_airports_with_deleted(&self.db_pool).await?
        } else {
            queries::list_airports(&self.db_pool).await?
        };

        let airports = airports.into_iter().map(Into::into).collect();
        Ok(Response::new(ListAirportsResponse { airports }))
    }

    async fn get_airport(
        &self,
        request: Request<GetAirportRequest>,
    ) -> Result<Response<Airport>, Status> {
        let GetAirportRequest { id } = request.into_inner();
        let id = parse_id(&id)?;

        let airport = queries::get_airport(&self.db_pool, &id).await?.into();

        Ok(Response::new(airport))
    }

    async fn create_airport(
        &self,
        request: Request<CreateAirportRequest>,
    ) -> std::result::Result<Response<Airport>, Status> {
        let Airport {
            icao,
            iata,
            name,
            country,
            city,
            ..
        } = request.into_inner().airport.unwrap_or_default();

        let airport = queries::create_airport(&self.db_pool, icao, iata, name, country, city)
            .await?
            .into();

        Ok(Response::new(airport))
    }

    async fn delete_airport(
        &self,
        request: Request<DeleteAirportRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let DeleteAirportRequest { id } = request.into_inner();
        let id = parse_id(&id)?;

        queries::delete_airport(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }
}

impl AirportsApp {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
