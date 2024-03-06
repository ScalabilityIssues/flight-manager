use sqlx::types::time::OffsetDateTime;
use sqlx::types::Uuid;
use sqlx::PgConnection;

type Result<T> = std::result::Result<T, crate::db::QueryError>;

pub struct Flight {
    pub id: Uuid,
    pub plane_id: Uuid,
    pub origin_id: Uuid,
    pub destination_id: Uuid,
    pub departure_time: OffsetDateTime,
    pub arrival_time: OffsetDateTime,
}

pub async fn list_flights(ex: &mut PgConnection) -> Result<Vec<Flight>> {
    let flights = sqlx::query_as!(
        Flight,
        "select * from flights where id not in (select flight_id from flight_cancellations)"
    )
    .fetch_all(ex)
    .await?;

    Ok(flights)
}

pub async fn list_flights_with_cancelled(ex: &mut PgConnection) -> Result<Vec<Flight>> {
    let flights = sqlx::query_as!(Flight, "select * from flights")
        .fetch_all(ex)
        .await?;

    Ok(flights)
}

pub async fn get_flight(ex: &mut PgConnection, id: &Uuid) -> Result<Flight> {
    let flight = sqlx::query_as!(Flight, "select * from flights where id = $1", id)
        .fetch_one(ex)
        .await?;

    Ok(flight)
}

pub async fn create_flight(
    ex: &mut PgConnection,
    plane_id: Uuid,
    origin_id: Uuid,
    destination_id: Uuid,
    departure_time: OffsetDateTime,
    arrival_time: OffsetDateTime,
) -> Result<Flight> {
    let flight = sqlx::query_as!(
        Flight,
        "insert into flights (id, plane_id, origin_id, destination_id, departure_time, arrival_time) values (gen_random_uuid(), $1, $2, $3, $4, $5) returning *",
        plane_id,
        origin_id,
        destination_id,
        departure_time,
        arrival_time
    )
    .fetch_one(ex)
    .await?;

    Ok(flight)
}

pub struct EventCancelled {
    pub flight_id: Uuid,
    pub timestamp: OffsetDateTime,
    pub reason: Option<String>,
}

pub async fn get_event_cancelled(
    ex: &mut PgConnection,
    id: &[Uuid],
) -> Result<Vec<EventCancelled>> {
    let events = sqlx::query_as!(
        EventCancelled,
        "select flight_cancellations.* from flight_cancellations join unnest($1::uuid[]) as U(ids) on flight_id = ids",
        id
    )
    .fetch_all(ex)
    .await?;

    Ok(events)
}

pub async fn add_event_cancelled(
    ex: &mut PgConnection,
    id: &Uuid,
    reason: String,
) -> Result<EventCancelled> {
    let e = sqlx::query_as!(
        EventCancelled,
        "insert into flight_cancellations (flight_id, reason) values ($1, $2) returning *",
        id,
        reason
    )
    .fetch_one(ex)
    .await?;

    Ok(e)
}

pub struct EventDelayed {
    pub flight_id: Uuid,
    pub timestamp: OffsetDateTime,
    pub departure_time: OffsetDateTime,
    pub arrival_time: OffsetDateTime,
}

pub async fn get_event_delayed(ex: &mut PgConnection, id: &[Uuid]) -> Result<Vec<EventDelayed>> {
    let events = sqlx::query_as!(
        EventDelayed,
        "select flight_delays.* from flight_delays join unnest($1::uuid[]) as U(ids) on flight_id = ids",
        id
    ).fetch_all(ex).await?;

    Ok(events)
}

pub async fn add_event_delayed(
    ex: &mut PgConnection,
    id: &Uuid,
    departure_time: &OffsetDateTime,
    arrival_time: &OffsetDateTime,
) -> Result<EventDelayed> {
    let e = sqlx::query_as!(
        EventDelayed,
        "insert into flight_delays (flight_id, departure_time, arrival_time) values ($1, $2, $3) returning *",
        id,
        departure_time,
        arrival_time
    )
    .fetch_one(ex)
    .await?;

    Ok(e)
}

pub struct EventGateDepartureSet {
    pub flight_id: Uuid,
    pub timestamp: OffsetDateTime,
    pub gate: String,
}

pub async fn get_event_gate_dep(
    ex: &mut PgConnection,
    id: &[Uuid],
) -> Result<Vec<EventGateDepartureSet>> {
    let events = sqlx::query_as!(
        EventGateDepartureSet,
        "select flight_departure_gates.* from flight_departure_gates join unnest($1::uuid[]) as U(ids) on flight_id = ids",
        id
    ).fetch_all(ex).await?;

    Ok(events)
}

pub async fn add_event_gate_dep_set(
    ex: &mut PgConnection,
    id: &Uuid,
    gate: &str,
) -> Result<EventGateDepartureSet> {
    let e = sqlx::query_as!(
        EventGateDepartureSet,
        "insert into flight_departure_gates (flight_id, gate) values ($1, $2) returning *",
        id,
        gate
    )
    .fetch_one(ex)
    .await?;

    Ok(e)
}

pub struct EventGateArrivalSet {
    pub flight_id: Uuid,
    pub timestamp: OffsetDateTime,
    pub gate: String,
}

pub async fn get_event_gate_arr(
    ex: &mut PgConnection,
    id: &[Uuid],
) -> Result<Vec<EventGateArrivalSet>> {
    let events = sqlx::query_as!(
        EventGateArrivalSet,
        "select flight_arrival_gates.* from flight_arrival_gates join unnest($1::uuid[]) as U(ids) on flight_id = ids",
        id
    ).fetch_all(ex).await?;

    Ok(events)
}

pub async fn add_event_gate_arr_set(
    ex: &mut PgConnection,
    id: &Uuid,
    gate: &str,
) -> Result<EventGateArrivalSet> {
    let e = sqlx::query_as!(
        EventGateArrivalSet,
        "insert into flight_arrival_gates (flight_id, gate) values ($1, $2) returning *",
        id,
        gate
    )
    .fetch_one(ex)
    .await?;

    Ok(e)
}
