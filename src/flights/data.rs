use std::collections::HashMap;

use itertools::Itertools;
use sqlx::{types::Uuid, PgConnection};
use time::OffsetDateTime;

use super::queries;

type Result<T> = std::result::Result<T, crate::db::DatabaseError>;

pub struct FlightData(
    pub queries::Flight,
    pub Vec<queries::EventCancelled>,
    pub Vec<queries::EventDelayed>,
    pub Vec<queries::EventGateDepartureSet>,
    pub Vec<queries::EventGateArrivalSet>,
);

fn group_by_id<T>(list: Vec<T>, id: &'static impl Fn(&T) -> Uuid) -> HashMap<Uuid, Vec<T>> {
    list.into_iter()
        .sorted_by_key(id)
        .group_by(id)
        .into_iter()
        .map(|(k, v)| (k, v.collect()))
        .collect()
}

pub async fn load_flights_data(
    ex: &mut PgConnection,
    flights: Vec<queries::Flight>,
) -> Result<impl Iterator<Item = FlightData>> {
    let ids = flights.iter().map(|f| f.id).collect::<Vec<_>>();

    let cancelled = queries::get_event_cancelled(ex, &ids).await?;
    let mut cancelled = group_by_id(cancelled, &|e| e.flight_id);

    let delayed = queries::get_event_delayed(ex, &ids).await?;
    let mut delayed = group_by_id(delayed, &|e| e.flight_id);

    let gate_dep = queries::get_event_gate_dep(ex, &ids).await?;
    let mut gate_dep = group_by_id(gate_dep, &|e| e.flight_id);

    let gate_arr = queries::get_event_gate_arr(ex, &ids).await?;
    let mut gate_arr = group_by_id(gate_arr, &|e| e.flight_id);

    let flights = flights.into_iter().map(move |f| {
        let id = f.id;
        let cancelled = cancelled.remove(&id).unwrap_or_default();
        let delayed = delayed.remove(&id).unwrap_or_default();
        let gate_dep = gate_dep.remove(&id).unwrap_or_default();
        let gate_arr = gate_arr.remove(&id).unwrap_or_default();
        FlightData(f, cancelled, delayed, gate_dep, gate_arr)
    });

    Ok(flights)
}

pub async fn list_flights(
    ex: &mut PgConnection,
    include_cancelled: bool,
) -> Result<impl Iterator<Item = FlightData>> {
    let flights = if include_cancelled {
        queries::list_flights_with_cancelled(ex).await?
    } else {
        queries::list_flights(ex).await?
    };

    load_flights_data(ex, flights).await
}

pub async fn search_flights(
    ex: &mut PgConnection,
    origin_airport_id: Uuid,
    destination_airport_id: Uuid,
    date: OffsetDateTime,
) -> Result<impl Iterator<Item = FlightData>> {
    let flights =
        queries::search_flights(ex, origin_airport_id, destination_airport_id, date).await?;

    load_flights_data(ex, flights).await
}

pub async fn get_flight(ex: &mut PgConnection, id: Uuid) -> Result<FlightData> {
    let flight = queries::get_flight(ex, &id).await?;

    let cancelled = queries::get_event_cancelled(ex, &[id]).await?;
    let delayed = queries::get_event_delayed(ex, &[id]).await?;
    let gate_dep = queries::get_event_gate_dep(ex, &[id]).await?;
    let gate_arr = queries::get_event_gate_arr(ex, &[id]).await?;

    Ok(FlightData(flight, cancelled, delayed, gate_dep, gate_arr))
}

pub async fn create_flight(
    ex: &mut PgConnection,
    plane_id: Uuid,
    origin_id: Uuid,
    destination_id: Uuid,
    departure_time: time::OffsetDateTime,
    arrival_time: time::OffsetDateTime,
) -> Result<FlightData> {
    let flight = queries::create_flight(
        ex,
        plane_id,
        origin_id,
        destination_id,
        departure_time,
        arrival_time,
    )
    .await?;

    Ok(FlightData(flight, vec![], vec![], vec![], vec![]))
}
