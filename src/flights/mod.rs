use std::ops::DerefMut;

use prost_types::Timestamp;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::parse::{parse_id, parse_timestamp, parse_update_paths};
use crate::proto::flightmngr::{
    flights_server::Flights, Flight, FlightList, FlightQuery, FlightUpdate,
};

mod queries;

#[derive(Debug)]
pub struct FlightsApp {
    db_pool: PgPool,
}

impl From<queries::Flight> for Flight {
    fn from(flight: queries::Flight) -> Self {
        Self {
            id: flight.id.to_string(),
            plane_id: flight.plane_id.to_string(),
            origin_id: flight.origin_id.to_string(),
            destination_id: flight.destination_id.to_string(),
            departure_time: Some(Timestamp {
                seconds: flight.departure_time.unix_timestamp(),
                nanos: flight.departure_time.nanosecond() as i32,
            }),
            arrival_time: Some(Timestamp {
                seconds: flight.arrival_time.unix_timestamp(),
                nanos: flight.arrival_time.nanosecond() as i32,
            }),
            departure_gate: flight.departure_gate,
            arrival_gate: flight.arrival_gate,
            status: flight.status,
        }
    }
}

#[tonic::async_trait]
impl Flights for FlightsApp {
    async fn list_flights(&self, _request: Request<()>) -> Result<Response<FlightList>, Status> {
        let flights = queries::list_flights(&self.db_pool).await?;
        let flights = flights.into_iter().map(Into::into).collect();

        Ok(Response::new(FlightList { flights }))
    }

    async fn get_flight(&self, request: Request<FlightQuery>) -> Result<Response<Flight>, Status> {
        let FlightQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        let flight = queries::get_flight(&self.db_pool, &id).await?.into();

        Ok(Response::new(flight))
    }

    async fn create_flight(
        &self,
        request: Request<Flight>,
    ) -> std::result::Result<Response<Flight>, Status> {
        let Flight {
            id: _,
            plane_id,
            origin_id,
            destination_id,
            departure_time,
            arrival_time,
            departure_gate,
            arrival_gate,
            status,
        } = request.into_inner();

        let plane_id = parse_id(&plane_id)?;
        let origin_id = parse_id(&origin_id)?;
        let destination_id = parse_id(&destination_id)?;
        let departure_time = parse_timestamp(&departure_time)?;
        let arrival_time = parse_timestamp(&arrival_time)?;

        let flight = queries::create_flight(
            &self.db_pool,
            plane_id,
            origin_id,
            destination_id,
            departure_time,
            arrival_time,
            departure_gate,
            arrival_gate,
            status,
        )
        .await?
        .into();

        Ok(Response::new(flight))
    }

    async fn delete_flight(
        &self,
        request: Request<FlightQuery>,
    ) -> std::result::Result<Response<()>, Status> {
        let FlightQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        queries::delete_flight(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }

    async fn update_flight(
        &self,
        request: Request<FlightUpdate>,
    ) -> std::result::Result<Response<Flight>, Status> {
        let FlightUpdate {
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

        let Flight {
            id: _,
            plane_id,
            origin_id,
            destination_id,
            departure_time,
            arrival_time,
            departure_gate,
            arrival_gate,
            status,
        } = update;

        for path in update_paths {
            match path.as_str() {
                "plane_id" => {
                    let plane_id = parse_id(&plane_id)?;
                    queries::update_plane_id(t.deref_mut(), &id, &plane_id).await?;
                }
                "origin_id" => {
                    let origin_id = parse_id(&origin_id)?;
                    queries::update_origin_id(t.deref_mut(), &id, &origin_id).await?;
                }
                "destination_id" => {
                    let destination_id = parse_id(&destination_id)?;
                    queries::update_destination_id(t.deref_mut(), &id, &destination_id).await?;
                }
                "departure_time" => {
                    let departure_time = parse_timestamp(&departure_time)?;
                    queries::update_departure_time(t.deref_mut(), &id, &departure_time).await?;
                }
                "arrival_time" => {
                    let arrival_time = parse_timestamp(&arrival_time)?;
                    queries::update_arrival_time(t.deref_mut(), &id, &arrival_time).await?;
                }
                "departure_gate" => {
                    queries::update_departure_gate(t.deref_mut(), &id, &departure_gate).await?;
                }
                "arrival_gate" => {
                    queries::update_arrival_gate(t.deref_mut(), &id, &arrival_gate).await?;
                }
                "status" => queries::update_status(t.deref_mut(), &id, &status).await?,

                _ => {
                    return Err(Status::invalid_argument(format!(
                        "'update_mask' contains invalid path '{}'",
                        path
                    )))
                }
            }
        }

        let flight = queries::get_flight(t.deref_mut(), &id).await?.into();

        t.commit()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        Ok(Response::new(flight))
    }
}

impl FlightsApp {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
