use std::ops::DerefMut;

use sqlx::{types::Uuid, PgPool};
use tonic::{Request, Response, Status};

use crate::proto::flightmngr::{
    planes_server::Planes, PlaneCreate, PlaneList, PlaneQuery, PlaneRead, PlaneUpdate,
};

mod queries;

#[derive(Debug)]
pub struct PlanesApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl Planes for PlanesApp {
    async fn list_planes(&self, _request: Request<()>) -> Result<Response<PlaneList>, Status> {
        let planes = queries::list_planes(&self.db_pool).await?;

        Ok(Response::new(PlaneList { planes }))
    }

    async fn get_plane(&self, request: Request<PlaneQuery>) -> Result<Response<PlaneRead>, Status> {
        let PlaneQuery { id } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        let plane = queries::get_plane(&self.db_pool, &id).await?;

        Ok(Response::new(plane))
    }

    async fn create_plane(
        &self,
        request: Request<PlaneCreate>,
    ) -> std::result::Result<Response<PlaneRead>, Status> {
        let plane = queries::create_plane(&self.db_pool, &request.into_inner()).await?;

        Ok(Response::new(plane))
    }

    async fn delete_plane(
        &self,
        request: Request<PlaneQuery>,
    ) -> std::result::Result<Response<()>, Status> {
        let PlaneQuery { id } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        queries::delete_plane(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }

    async fn update_plane(
        &self,
        request: Request<PlaneUpdate>,
    ) -> std::result::Result<Response<PlaneRead>, Status> {
        let PlaneUpdate { id, patch } = request.into_inner();
        let id = Uuid::try_parse(&id).map_err(|_| Status::invalid_argument("id"))?;

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        if let Some(patch) = patch {
            if let Some(name) = patch.name {
                queries::update_name(t.deref_mut(), &id, &name).await?;
            }
            if let Some(model) = patch.model {
                queries::update_model(t.deref_mut(), &id, &model).await?;
            }
            if let Some(cabin_capacity) = patch.cabin_capacity {
                queries::update_cabin_cap(t.deref_mut(), &id, cabin_capacity).await?;
            }
            if let Some(cargo_capacity_kg) = patch.cargo_capacity_kg {
                queries::update_cargo_cap(t.deref_mut(), &id, cargo_capacity_kg).await?;
            }
        }

        let plane = queries::get_plane(t.deref_mut(), &id).await?;

        t.commit()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        Ok(Response::new(plane))
    }
}

impl PlanesApp {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
