use flightmngr::proto::flightmngr::{
    Airport, CreateAirportRequest, DeleteAirportRequest, GetAirportRequest, ListAirportsRequest,
};
use sqlx::{types::Uuid, PgPool};

mod common;

fn example_airport_1() -> Airport {
    Airport {
        name: "Test Airport 1".to_string(),
        city: "Test City 1".to_string(),
        country: "Test Country 1".to_string(),
        iata: "TST".to_string(),
        icao: "TSTT".to_string(),
        id: Default::default(),
        deleted: false,
    }
}

fn example_airport_2() -> Airport {
    Airport {
        name: "Test Airport 2".to_string(),
        city: "Test City 2".to_string(),
        country: "Test Country 2".to_string(),
        iata: "TSS".to_string(),
        icao: "TSST".to_string(),
        id: Default::default(),
        deleted: false,
    }
}

#[sqlx::test]
fn list_create_get(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    // list empty
    let r = client
        .airports
        .list_airports(ListAirportsRequest { show_deleted: true })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 0);

    // create
    let r = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(example_airport_1()),
        })
        .await
        .unwrap()
        .into_inner();

    assert_ne!(r.id, "");
    assert_eq!(r.deleted, false);
    assert_eq!(r.name, "Test Airport 1");
    assert_eq!(r.city, "Test City 1");

    let id = r.id;

    // list 1
    let r = client
        .airports
        .list_airports(ListAirportsRequest {
            show_deleted: false,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 1);
    assert_eq!(r.airports[0].id, id);

    // get
    let r = client
        .airports
        .get_airport(GetAirportRequest { id: id.clone() })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.id, id);
    assert_eq!(r.name, "Test Airport 1");
    assert_eq!(r.city, "Test City 1");
}

#[sqlx::test]
async fn not_found(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    // get missing
    let r = client
        .airports
        .get_airport(GetAirportRequest {
            id: Uuid::default().to_string(),
        })
        .await;

    assert!(r.is_err_and(|e| e.code() == tonic::Code::NotFound));
}

#[sqlx::test]
async fn delete(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    // create
    let r = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(example_airport_1()),
        })
        .await
        .unwrap()
        .into_inner();

    let id = r.id;
    assert_eq!(r.deleted, false);

    // delete
    let _ = client
        .airports
        .delete_airport(DeleteAirportRequest { id: id.clone() })
        .await
        .unwrap();

    // get
    let r = client
        .airports
        .get_airport(GetAirportRequest { id: id.clone() })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.id, id);
    assert_eq!(r.deleted, true);
}

#[sqlx::test]
async fn show_deleted(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    // create first
    let r = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(example_airport_1()),
        })
        .await
        .unwrap()
        .into_inner();

    let id1 = r.id;

    // create second
    let r = client
        .airports
        .create_airport(CreateAirportRequest {
            airport: Some(example_airport_2()),
        })
        .await
        .unwrap()
        .into_inner();

    let id2 = r.id;

    // list all
    let r = client
        .airports
        .list_airports(ListAirportsRequest {
            show_deleted: false,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 2);
    assert!(r.airports.iter().all(|a| !a.deleted));

    // list
    let r = client
        .airports
        .list_airports(ListAirportsRequest { show_deleted: true })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 2);

    // delete first
    let _ = client
        .airports
        .delete_airport(DeleteAirportRequest { id: id1.clone() })
        .await
        .unwrap();

    // list
    let r = client
        .airports
        .list_airports(ListAirportsRequest {
            show_deleted: false,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 1);
    assert_eq!(r.airports[0].id, id2);
    assert!(r.airports.iter().all(|a| !a.deleted));

    // list deleted
    let r = client
        .airports
        .list_airports(ListAirportsRequest { show_deleted: true })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 2);

    // delete second
    let _ = client
        .airports
        .delete_airport(DeleteAirportRequest { id: id2.clone() })
        .await
        .unwrap();

    // list
    let r = client
        .airports
        .list_airports(ListAirportsRequest {
            show_deleted: false,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 0);
    assert!(r.airports.iter().all(|a| !a.deleted));

    // list deleted
    let r = client
        .airports
        .list_airports(ListAirportsRequest { show_deleted: true })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(r.airports.len(), 2);
    assert!(r.airports.iter().all(|a| a.deleted));
}
