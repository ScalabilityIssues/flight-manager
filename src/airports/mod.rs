use tonic::{Request, Response, Status};

use crate::{
    datautils::parse_id,
    db::Database,
    proto::flightmngr::{
        airports_server::Airports, Airport, CreateAirportRequest, DeleteAirportRequest,
        GetAirportRequest, ListAirportsRequest, ListAirportsResponse,
    },
};

mod map;
mod queries;

pub struct AirportsApp {
    db: Database,
}

#[tonic::async_trait]
impl Airports for AirportsApp {
    async fn list_airports(
        &self,
        request: Request<ListAirportsRequest>,
    ) -> Result<Response<ListAirportsResponse>, Status> {
        let ListAirportsRequest { show_deleted } = request.into_inner();
        let mut t = self.db.begin().await?;

        let airports = if show_deleted {
            queries::list_airports_with_deleted(t.get_conn()).await?
        } else {
            queries::list_airports(t.get_conn()).await?
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
        let mut t = self.db.begin().await?;

        let airport = queries::get_airport(t.get_conn(), &id).await?.into();

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
        let mut t = self.db.begin().await?;

        let airport = queries::create_airport(t.get_conn(), icao, iata, name, country, city)
            .await?
            .into();

        t.commit().await?;
        Ok(Response::new(airport))
    }

    async fn delete_airport(
        &self,
        request: Request<DeleteAirportRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let DeleteAirportRequest { id } = request.into_inner();
        let id = parse_id(&id)?;
        let mut t = self.db.begin().await?;

        queries::delete_airport(t.get_conn(), &id).await?;

        t.commit().await?;
        Ok(Response::new(()))
    }
}

impl AirportsApp {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}
