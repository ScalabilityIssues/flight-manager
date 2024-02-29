use sqlx::PgPool;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tower::service_fn;

use flightmngr::proto::flightmngr::{
    airports_client::AirportsClient, flights_client::FlightsClient, planes_client::PlanesClient,
};

pub struct Clients {
    pub airports: AirportsClient<Channel>,
    pub planes: PlanesClient<Channel>,
    pub flights: FlightsClient<Channel>,
}

pub async fn make_test_client(db: PgPool) -> Result<Clients, Box<dyn std::error::Error>> {
    let (client, server) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        let result = Server::builder()
            .add_routes(flightmngr::build_services(&db))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await;
        assert!(result.is_ok());
    });

    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")? // The URL will be ignored.
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client already taken",
                    ))
                }
            }
        }))
        .await?;

    let clients = Clients {
        airports: AirportsClient::new(channel.clone()),
        planes: PlanesClient::new(channel.clone()),
        flights: FlightsClient::new(channel),
    };

    Ok(clients)
}
