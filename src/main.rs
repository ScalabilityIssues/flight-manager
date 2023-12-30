use std::net::{IpAddr, SocketAddr};

use anyhow;
use clap::Parser;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tower_http::{cors, trace};
use tracing::Level;

use crate::airports::AirportsApp;
use crate::planes::PlanesApp;
use crate::proto::flightmngr::airports_server::AirportsServer;
use crate::proto::flightmngr::planes_server::PlanesServer;

mod airports;
mod planes;
mod proto;

#[derive(clap::Parser, Debug)]
struct Opt {
    #[clap(env = "DATABASE_URL")]
    db: String,
    #[clap(long, default_value = "0.0.0.0")]
    ip: IpAddr,
    #[clap(long, default_value = "50051")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let opt = Opt::parse();

    let db_pool = PgPool::connect(&opt.db).await?;

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

    let cors = cors::CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([http::Method::POST])
        .allow_origin(["http://localhost:3000".parse()?]);

    Server::builder()
        // configure the server
        .accept_http1(true)
        .timeout(std::time::Duration::from_secs(10))
        .layer(cors)
        .layer(
            trace::TraceLayer::new_for_grpc()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(tonic_web::GrpcWebLayer::new())
        // cnable grpc reflection
        .add_service(reflection)
        // add services
        .add_service(PlanesServer::new(PlanesApp::new(db_pool.clone())))
        .add_service(AirportsServer::new(AirportsApp::new(db_pool)))
        // serve
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = signal(SignalKind::terminate()).unwrap().recv().await;
            tracing::info!("shutting down");
        })
        .await?;

    Ok(())
}
