use flightmngr::proto::flightmngr::{
    Airport, CreateAirportRequest, CreateFlightRequest, CreatePlaneRequest, Flight,
    GetFlightRequest, Plane,
};
use sqlx::PgPool;

mod common;

fn default_flight(plane_id: String, origin_id: String, destination_id: String) -> Flight {
    Flight {
        plane_id,
        origin_id,
        destination_id,
        departure_time: Some(Default::default()),
        arrival_time: Some(Default::default()),
        id: Default::default(),
        status_events: Default::default(),
        is_cancelled: Default::default(),
        expected_departure_time: Default::default(),
        expected_arrival_time: Default::default(),
        departure_gate: Default::default(),
        arrival_gate: Default::default(),
    }
}

fn default_plane() -> Plane {
    Plane {
        id: Default::default(),
        deleted: false,
        model: "Test Model".to_string(),
        cabin_capacity: 200,
        cargo_capacity_kg: 1000,
    }
}

fn default_airport() -> Airport {
    Airport {
        id: Default::default(),
        deleted: false,
        name: "Test Airport".to_string(),
        city: "Test City".to_string(),
        country: "Test Country".to_string(),
        iata: "TST".to_string(),
        icao: "TSTT".to_string(),
    }
}

#[sqlx::test]
async fn test(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    let airport1 = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(default_airport()),
        })
        .await
        .unwrap()
        .into_inner();

    let airport2 = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(default_airport()),
        })
        .await
        .unwrap()
        .into_inner();

    let plane = client
        .planes
        .create_plane(CreatePlaneRequest {
            plane: Some(default_plane()),
        })
        .await
        .unwrap()
        .into_inner();

    let flight = client
        .flights
        .create_flight(CreateFlightRequest {
            flight: Some(default_flight(plane.id, airport1.id, airport2.id)),
        })
        .await
        .unwrap()
        .into_inner();

    let r = client
        .flights
        .get_flight(GetFlightRequest { id: flight.id.clone() })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r, flight);
}
