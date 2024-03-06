use std::net::SocketAddr;

use sqlx::PgPool;

use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tower_http::trace;
use tracing::Level;

mod config;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let opt = envy::from_env::<config::Options>()?;

    let db_pool = PgPool::connect(&opt.database_url).await?;

    // run migrations
    tracing::info!("running migrations");
    flightmngr::db::MIGRATOR.run(&db_pool).await?;

    // build grpc services
    let services = flightmngr::build_services(db_pool);
    // build reflection service
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(flightmngr::proto::FILE_DESCRIPTOR_SET)
        .build()?;

    // bind server socket
    let address = SocketAddr::new(opt.ip, opt.port);
    let listener = TcpListener::bind(address).await?;
    tracing::info!("starting server on {}", address);
    
    // run server
    Server::builder()
        // configure the server
        .layer(
            trace::TraceLayer::new_for_grpc()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // add services
        .add_routes(services)
        // enable grpc reflection
        .add_service(reflection)
        // serve
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = signal(SignalKind::terminate()).unwrap().recv().await;
            tracing::info!("shutting down");
        })
        .await?;

    Ok(())
}
