use sqlx::PgPool;
use std::future::Future;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
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

pub async fn server_and_client_stub(db: PgPool) -> (impl Future<Output = ()>, Clients) {
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    std::fs::remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let serve_future = async move {
        let result = Server::builder()
            .add_routes(flightmngr::build_services(&db))
            .serve_with_incoming(stream)
            .await;
        assert!(result.is_ok());
    };

    let socket = Arc::clone(&socket);
    // Connect to the server over a Unix socket
    // The URL will be ignored.
    let channel = Endpoint::try_from("http://any.url")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket = Arc::clone(&socket);
            async move { UnixStream::connect(&*socket).await }
        }))
        .await
        .unwrap();

    let clients = Clients {
        airports: AirportsClient::new(channel.clone()),
        planes: PlanesClient::new(channel.clone()),
        flights: FlightsClient::new(channel),
    };

    (serve_future, clients)
}
