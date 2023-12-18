use anyhow;
use clap::Parser;
use sqlx::PgPool;

use futures::stream::StreamExt;
use planes::Empty;
use planes::{planes_server::Planes, PlaneRead};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::planes::planes_server::PlanesServer;

mod planes {
    tonic::include_proto!("planes");
}

#[derive(Parser, Debug)]
struct Opt {
    db: String,
}

// #[derive(Debug)]
// struct Plane {
//     id: String,
//     name: String,
//     capacity: i32,
// }

async fn list_planes(pool: &PgPool) -> anyhow::Result<Vec<PlaneRead>> {
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
    type ListPlanesStream = ReceiverStream<Result<PlaneRead, Status>>;

    async fn list_planes(
        &self,
        req: Request<Empty>,
    ) -> Result<Response<Self::ListPlanesStream>, Status> {
        println!("Got a request: {:?}", req);
        let (tx, rx) = mpsc::channel(4);
        let pool = self.db_pool.clone();

        tokio::spawn(async move {
            let mut planes = sqlx::query_as!(PlaneRead, "SELECT * FROM planes").fetch(&pool);

            while let Some(plane) = planes.next().await {
                tx.send(plane.map_err(|err| Status::new(tonic::Code::Internal, err.to_string())))
                    .await
                    .unwrap();
            }
        });

        Ok(Response::new(Self::ListPlanesStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    println!("{:?}", opt);

    let pool = PgPool::connect(&opt.db).await?;

    let planes = list_planes(&pool).await?;

    println!("planes: {:?}", planes);

    let state = MyPlanes { db_pool: pool };

    Server::builder()
        .add_service(PlanesServer::new(state))
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
