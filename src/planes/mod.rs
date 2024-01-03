use std::ops::DerefMut;

use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    parse::{parse_id, parse_update_paths},
    proto::flightmngr::{planes_server::Planes, Plane, PlaneList, PlaneQuery, PlaneUpdate},
};

mod queries;

#[derive(Debug)]
pub struct PlanesApp {
    db_pool: PgPool,
}

impl From<queries::Plane> for Plane {
    fn from(plane: queries::Plane) -> Self {
        Self {
            id: plane.id.to_string(),
            name: plane.name,
            model: plane.model,
            cabin_capacity: plane.cabin_capacity,
            cargo_capacity_kg: plane.cargo_capacity_kg,
        }
    }
}

#[tonic::async_trait]
impl Planes for PlanesApp {
    async fn list_planes(&self, _request: Request<()>) -> Result<Response<PlaneList>, Status> {
        let planes = queries::list_planes(&self.db_pool).await?;
        let planes = planes.into_iter().map(Into::into).collect();

        Ok(Response::new(PlaneList { planes }))
    }

    async fn get_plane(&self, request: Request<PlaneQuery>) -> Result<Response<Plane>, Status> {
        let PlaneQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        let plane = queries::get_plane(&self.db_pool, &id).await?.into();

        Ok(Response::new(plane))
    }

    async fn create_plane(
        &self,
        request: Request<Plane>,
    ) -> std::result::Result<Response<Plane>, Status> {
        let Plane {
            id: _,
            name,
            model,
            cabin_capacity,
            cargo_capacity_kg,
        } = request.into_inner();

        let plane = queries::create_plane(
            &self.db_pool,
            name,
            model,
            cabin_capacity,
            cargo_capacity_kg,
        )
        .await?
        .into();

        Ok(Response::new(plane))
    }

    async fn delete_plane(
        &self,
        request: Request<PlaneQuery>,
    ) -> std::result::Result<Response<()>, Status> {
        let PlaneQuery { id } = request.into_inner();
        let id = parse_id(&id)?;

        queries::delete_plane(&self.db_pool, &id).await?;

        Ok(Response::new(()))
    }

    async fn update_plane(
        &self,
        request: Request<PlaneUpdate>,
    ) -> std::result::Result<Response<Plane>, Status> {
        let PlaneUpdate {
            id,
            update,
            update_mask,
        } = request.into_inner();
        let id = parse_id(&id)?;
        let update_paths = parse_update_paths(update_mask)?;
        let update = update.unwrap_or_default();

        let mut t = self
            .db_pool
            .begin()
            .await
            .map_err(|err| Status::from_error(Box::new(err)))?;

        for path in update_paths {
            match path.as_str() {
                "name" => queries::update_name(t.deref_mut(), &id, &update.name).await?,
                "model" => queries::update_model(t.deref_mut(), &id, &update.model).await?,
                "cabin_capacity" => {
                    queries::update_cabin_cap(t.deref_mut(), &id, update.cabin_capacity).await?
                }
                "cargo_capacity_kg" => {
                    queries::update_cargo_cap(t.deref_mut(), &id, update.cargo_capacity_kg).await?
                }
                _ => {
                    return Err(Status::invalid_argument(format!(
                        "'update_mask' contains invalid path '{}'",
                        path
                    )));
                }
            }
        }

        let plane = queries::get_plane(t.deref_mut(), &id).await?.into();

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
