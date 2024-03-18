use std::net::SocketAddr;

use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tower_http::classify::{GrpcErrorsAsFailures, SharedClassifier};
use tower_http::trace;
use tracing::Level;

use proto;
mod config;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let opt = envy::from_env::<config::Options>()?;

    tracing::info!("connecting to database");
    let db_pool = PgPool::connect(&opt.database_url).await?;

    // run migrations
    tracing::info!("running migrations");
    flightmngr::db::MIGRATOR.run(&db_pool).await?;

    // build grpc services
    let services = flightmngr::build_services(db_pool);
    // build reflection service
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    // bind server socket
    let address = SocketAddr::new(opt.ip, opt.port);
    let listener = TcpListener::bind(address).await?;
    tracing::info!("starting server on {}", address);

    // run server
    Server::builder()
        // configure the server
        .layer(
            trace::TraceLayer::new(SharedClassifier::new(
                GrpcErrorsAsFailures::new().with_success(tower_http::classify::GrpcCode::NotFound),
            ))
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::ERROR))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // add services
        .add_routes(services)
        // enable grpc reflection
        .add_service(reflection)
        // serve
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = signal(SignalKind::terminate()).unwrap().recv().await;
        })
        .await?;

    Ok(())
}
