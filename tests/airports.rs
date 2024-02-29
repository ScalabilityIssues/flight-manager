use flightmngr::proto::flightmngr::ListAirportsRequest;
use sqlx::PgPool;

mod common;

#[sqlx::test]
fn test(db: PgPool) {
    let mut client = common::make_test_client(db).await.unwrap();

    let response = client
        .airports
        .list_airports(ListAirportsRequest { show_deleted: true })
        .await
        .unwrap();

    assert_eq!(response.into_inner().airports.len(), 0);
}
