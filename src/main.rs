use std::error::Error;

use anyhow;
use clap::Parser;
use sqlx::PgPool;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::planes::{
    planes_server::Planes, planes_server::PlanesServer, Empty, PlaneList, PlaneRead,
};

mod planes {
    tonic::include_proto!("planes");
}

#[derive(Parser, Debug)]
struct Opt {
    #[clap(env = "DATABASE_URL")]
    db: String,
}

async fn list_planes(
    pool: &PgPool,
) -> Result<Vec<PlaneRead>, Box<dyn Error + Send + Sync + 'static>> {
    let planes = sqlx::query_as!(PlaneRead, "SELECT * FROM planes")
        .fetch_all(pool)
        .await?;

    Ok(planes)
}

struct MyPlanes {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl Planes for MyPlanes {
    async fn list_planes(&self, req: Request<Empty>) -> Result<Response<PlaneList>, Status> {
        println!("Got a request: {:?}", req);

        let planes = list_planes(&self.db_pool)
            .await
            .map_err(Status::from_error)?;

        Ok(Response::new(PlaneList { planes }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    println!("{:?}", opt);

    let db_pool = PgPool::connect(&opt.db).await?;
    let state = MyPlanes { db_pool };

    Server::builder()
        .add_service(PlanesServer::new(state))
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
