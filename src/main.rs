use std::net::{IpAddr, SocketAddr};

use anyhow;
use clap::Parser;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

use crate::planes::{PlanesApp, PlanesServer};

mod planes;

#[derive(clap::Parser, Debug)]
struct Opt {
    #[clap(env = "DATABASE_URL")]
    db: String,
    #[clap(long, default_value = "127.0.0.1")]
    ip: IpAddr,
    #[clap(long, default_value = "50051")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

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
        .register_encoded_file_descriptor_set(planes::proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let cors = tower_http::cors::CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods([http::Method::POST])
        .allow_origin(["http://localhost:3000".parse()?]);

    Server::builder()
        // configure the server
        .accept_http1(true)
        .timeout(std::time::Duration::from_secs(10))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .layer(tonic_web::GrpcWebLayer::new())
        // cnable grpc reflection
        .add_service(reflection)
        // add services
        .add_service(PlanesServer::new(PlanesApp::new(db_pool)))
        // serve
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
