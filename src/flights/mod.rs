use tonic::{Request, Response, Status};

use crate::datautils::{parse_id, parse_timestamp};
use crate::db::Database;
use crate::proto::flightmngr::flight_status_event::Event;
use crate::proto::flightmngr::{
    flights_server::Flights, CreateFlightRequest, Flight, GetFlightRequest, ListFlightsRequest,
    ListFlightsResponse, SearchFlightsRequest, UpdateFlightRequest,
};
use crate::proto::flightmngr::{
    FlightCancelled, FlightDelayed, FlightGateArrival, FlightGateDeparture, FlightStatusEvent,
};

use crate::rabbitmq::Rabbit;
mod data;
mod map;
mod queries;

pub struct FlightsApp {
    db: Database,
    rabbitmq: Rabbit,
}

#[tonic::async_trait]
impl Flights for FlightsApp {
    async fn list_flights(
        &self,
        request: Request<ListFlightsRequest>,
    ) -> Result<Response<ListFlightsResponse>, Status> {
        let ListFlightsRequest { include_cancelled } = request.into_inner();
        let mut t = self.db.begin().await?;

        let flights = data::list_flights(t.get_conn(), include_cancelled).await?;

        let flights = flights.map(Into::into).collect();
        Ok(Response::new(ListFlightsResponse { flights }))
    }

    async fn search_flights(
        &self,
        request: Request<SearchFlightsRequest>,
    ) -> Result<Response<ListFlightsResponse>, Status> {
        let SearchFlightsRequest {
            origin_id,
            destination_id,
            departure_day,
        } = request.into_inner();

        let origin_id = parse_id(&origin_id)?;
        let destination_id = parse_id(&destination_id)?;
        let departure_day = parse_timestamp(departure_day)?;

        let mut t = self.db.begin().await?;

        let flights =
            data::search_flights(t.get_conn(), origin_id, destination_id, departure_day).await?;

        let flights = flights.map(Into::into).collect();
        Ok(Response::new(ListFlightsResponse { flights }))
    }

    async fn get_flight(
        &self,
        request: Request<GetFlightRequest>,
    ) -> Result<Response<Flight>, Status> {
        let GetFlightRequest { id } = request.into_inner();
        let id = parse_id(&id)?;
        let mut t = self.db.begin().await?;

        let flight = data::get_flight(t.get_conn(), id).await?;

        Ok(Response::new(flight.into()))
    }

    async fn create_flight(
        &self,
        request: Request<CreateFlightRequest>,
    ) -> std::result::Result<Response<Flight>, Status> {
        let Flight {
            plane_id,
            origin_id,
            destination_id,
            departure_time,
            arrival_time,
            ..
        } = request.into_inner().flight.unwrap_or_default();

        let plane_id = parse_id(&plane_id)?;
        let origin_id = parse_id(&origin_id)?;
        let destination_id = parse_id(&destination_id)?;
        let departure_time = parse_timestamp(departure_time)?;
        let arrival_time = parse_timestamp(arrival_time)?;

        let mut t = self.db.begin().await?;

        let flight = data::create_flight(
            t.get_conn(),
            plane_id,
            origin_id,
            destination_id,
            departure_time,
            arrival_time,
        )
        .await?
        .into();

        t.commit().await?;
        Ok(Response::new(flight))
    }

    async fn update_flight(
        &self,
        request: Request<UpdateFlightRequest>,
    ) -> std::result::Result<Response<Flight>, Status> {
        let UpdateFlightRequest { id, status_event } = request.into_inner();
        let id = parse_id(&id)?;
        let FlightStatusEvent { event, .. } =
            status_event.ok_or(Status::invalid_argument("'status_event' is required"))?;
        let event = event.ok_or(Status::invalid_argument("'status_event.event' is required"))?;

        let mut t = self.db.begin().await?;

        match event {
            Event::FlightCancelled(FlightCancelled { reason }) => {
                queries::add_event_cancelled(t.get_conn(), &id, reason).await?;
            }
            Event::FlightDelayed(FlightDelayed {
                arrival_time,
                departure_time,
            }) => {
                let arrival_time = parse_timestamp(arrival_time)?;
                let departure_time = parse_timestamp(departure_time)?;
                queries::add_event_delayed(t.get_conn(), &id, &departure_time, &arrival_time)
                    .await?;
            }
            Event::FlightGateDeparture(FlightGateDeparture { gate }) => {
                queries::add_event_gate_dep_set(t.get_conn(), &id, &gate).await?;
            }
            Event::FlightGateArrival(FlightGateArrival { gate }) => {
                queries::add_event_gate_arr_set(t.get_conn(), &id, &gate).await?;
            }
        };

        let flight = data::get_flight(t.get_conn(), id).await?.into();
        t.commit().await?;

        self.rabbitmq.notify_flight_update(&flight).await?;

        Ok(Response::new(flight))
    }
}

impl FlightsApp {
    pub fn new(db: Database, rabbitmq: Rabbit) -> Self {
        Self { db, rabbitmq }
    }
}
