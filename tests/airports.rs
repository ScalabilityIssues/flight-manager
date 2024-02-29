use flightmngr::proto::flightmngr::ListAirportsRequest;
use sqlx::PgPool;

mod common;

#[sqlx::test]
fn test(db: PgPool) {
    let (server_future, mut client) = common::server_and_client_stub(db).await;

    let request_future = async {
        let response = client
            .airports
            .list_airports(ListAirportsRequest { show_deleted: true })
            .await
            .unwrap();

        assert_eq!(response.into_inner().airports.len(), 0);
    };

    tokio::select! {
        _ = server_future => panic!("server failed"),
        _ = request_future => (),
    }
}
