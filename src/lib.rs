use db::Database;
use sqlx::PgPool;
use tonic::transport::server::Routes;

pub mod airports;
mod datautils;
pub mod db;
pub mod flights;
pub mod planes;

use crate::{airports::AirportsApp, flights::FlightsApp, planes::PlanesApp};

use proto::flightmngr::{
    airports_server::AirportsServer, flights_server::FlightsServer, planes_server::PlanesServer,
};

pub fn build_services(db_pool: PgPool) -> Routes {
    let db = Database::from_pool(db_pool);

    Routes::default()
        .add_service(PlanesServer::new(PlanesApp::new(db.clone())))
        .add_service(AirportsServer::new(AirportsApp::new(db.clone())))
        .add_service(FlightsServer::new(FlightsApp::new(db.clone())))
}
