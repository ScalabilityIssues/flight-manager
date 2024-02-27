use std::net::SocketAddr;

use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tower_http::trace;
use tracing::Level;

use crate::airports::AirportsApp;
use crate::flights::FlightsApp;
use crate::planes::PlanesApp;
use crate::proto::flightmngr::airports_server::AirportsServer;
use crate::proto::flightmngr::flights_server::FlightsServer;
use crate::proto::flightmngr::planes_server::PlanesServer;

mod airports;
mod config;
mod db;
mod datautils;
mod planes;
mod proto;
mod flights;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let opt = envy::from_env::<config::Options>()?;

    let db_pool = PgPool::connect(&opt.database_url).await?;

    // run migrations
    tracing::info!("running migrations");
    sqlx::migrate!().run(&db_pool).await?;

    // bind server socket
    let addr = SocketAddr::new(opt.ip, opt.port);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("starting server on {}", addr);

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        // configure the server
        .timeout(std::time::Duration::from_secs(10))
        .layer(
            trace::TraceLayer::new_for_grpc()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // cnable grpc reflection
        .add_service(reflection)
        // add services
        .add_service(PlanesServer::new(PlanesApp::new(db_pool.clone())))
        .add_service(AirportsServer::new(AirportsApp::new(db_pool.clone())))
        .add_service(FlightsServer::new(FlightsApp::new(db_pool)))
        // serve
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = signal(SignalKind::terminate()).unwrap().recv().await;
            tracing::info!("shutting down");
        })
        .await?;

    Ok(())
}
