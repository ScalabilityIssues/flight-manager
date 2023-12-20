use std::ops::DerefMut;

use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub use proto::planes_server::PlanesServer;

pub mod proto;
mod queries;

#[derive(Debug)]
pub struct PlanesApp {
    db_pool: PgPool,
}

#[tonic::async_trait]
impl proto::planes_server::Planes for PlanesApp {
    async fn list_planes(
        &self,
        _request: Request<proto::Empty>,
    ) -> Result<Response<proto::PlaneList>, Status> {
        let planes = queries::list_planes(&self.db_pool).await?;

        Ok(Response::new(proto::PlaneList { planes }))
    }

    async fn get_plane(
        &self,
        request: Request<proto::IdQuery>,
    ) -> Result<Response<proto::PlaneRead>, Status> {
        let id = request.into_inner().try_get_uuid()?;

        let plane = queries::get_plane(&self.db_pool, &id).await?;

        Ok(Response::new(plane))
    }

    async fn create_plane(
        &self,
        request: Request<proto::PlaneCreate>,
    ) -> std::result::Result<Response<proto::PlaneRead>, Status> {
        let plane = queries::create_plane(&self.db_pool, &request.into_inner()).await?;

        Ok(Response::new(plane))
    }

    async fn delete_plane(
        &self,
        request: Request<proto::IdQuery>,
    ) -> std::result::Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().try_get_uuid()?;

        queries::delete_plane(&self.db_pool, &id).await?;

        Ok(Response::new(proto::Empty {}))
    }

    async fn update_plane(
        &self,
        request: Request<proto::PlaneUpdate>,
    ) -> std::result::Result<Response<proto::PlaneRead>, Status> {
        let update = request.into_inner();
        let id = update.try_get_uuid()?;

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        if let Some(patch) = update.patch {
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
