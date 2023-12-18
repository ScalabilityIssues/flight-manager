use tokio_stream::StreamExt;

use crate::planes::{planes_client::PlanesClient, Empty};

pub mod planes {
    tonic::include_proto!("planes");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = PlanesClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Empty {});

    let response = client.list_planes(request).await?;
    let mut stream = response.into_inner();

    while let Some(Ok(plane)) = stream.next().await {
        println!("NOTE = {:?}", plane);
    }

    Ok(())
}
