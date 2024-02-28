use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    datautils::parse_id,
    proto::flightmngr::{
        planes_server::Planes, CreatePlaneRequest, DeletePlaneRequest, GetPlaneRequest,
        ListPlanesRequest, ListPlanesResponse, Plane,
    },
};

mod map;
mod queries;

#[derive(Debug)]
pub struct PlanesApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl Planes for PlanesApp {
    async fn list_planes(
        &self,
        request: Request<ListPlanesRequest>,
    ) -> Result<Response<ListPlanesResponse>, Status> {
        let ListPlanesRequest { show_deleted } = request.into_inner();

        let planes = if show_deleted {
            queries::list_planes_with_deleted(&self.db_pool).await?
        } else {
            queries::list_planes(&self.db_pool).await?
        };

        let planes = planes.into_iter().map(Into::into).collect();
        Ok(Response::new(ListPlanesResponse { planes }))
    }

    async fn get_plane(
        &self,
        request: Request<GetPlaneRequest>,
    ) -> Result<Response<Plane>, Status> {
        let GetPlaneRequest { id } = request.into_inner();
        let id = parse_id(&id)?;

        let plane = queries::get_plane(&self.db_pool, &id).await?.into();

        Ok(Response::new(plane))
    }

    async fn create_plane(
        &self,
        request: Request<CreatePlaneRequest>,
    ) -> std::result::Result<Response<Plane>, Status> {
        let Plane {
            model,
            cabin_capacity,
            cargo_capacity_kg,
            ..
        } = request.into_inner().plane.unwrap_or_default();

        let plane = queries::create_plane(
            &self.db_pool,
            model,
            cabin_capacity as i32,
            cargo_capacity_kg as i32,
        )
        .await?
        .into();

        Ok(Response::new(plane))
    }

    async fn delete_plane(
        &self,
        request: Request<DeletePlaneRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let DeletePlaneRequest { id } = request.into_inner();
        let id = parse_id(&id)?;

        queries::delete_plane(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }
}

impl PlanesApp {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
