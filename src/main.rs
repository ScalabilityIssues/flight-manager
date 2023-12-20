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
    tracing_subscriber::fmt().init();

    let opt = Opt::parse();

    let db_pool = PgPool::connect(&opt.db).await?;

    tracing::info!("running migrations");
    sqlx::migrate!().run(&db_pool).await?;

    let addr = SocketAddr::new(opt.ip, opt.port);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("starting server on {}", addr);

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(planes::proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .timeout(std::time::Duration::from_secs(10))
        // .layer(tower_http::trace::TraceLayer::new_for_grpc())  // <- broken
        .layer(tonic::service::interceptor(|req| {
            tracing::info!("received request {:?}", req);
            Ok(req)
        }))
        .add_service(reflection)
        .add_service(PlanesServer::new(PlanesApp::new(db_pool)))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
