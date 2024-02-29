use sqlx::PgPool;
use tonic::transport::server::Routes;

use crate::airports::AirportsApp;
use crate::flights::FlightsApp;
use crate::planes::PlanesApp;
use crate::proto::flightmngr::airports_server::AirportsServer;
use crate::proto::flightmngr::flights_server::FlightsServer;
use crate::proto::flightmngr::planes_server::PlanesServer;

pub mod airports;
mod datautils;
pub mod db;
pub mod flights;
pub mod planes;
pub mod proto;

pub fn build_services(db_pool: &PgPool) -> Routes {
    Routes::default()
        .add_service(PlanesServer::new(PlanesApp::new(db_pool.clone())))
        .add_service(AirportsServer::new(AirportsApp::new(db_pool.clone())))
        .add_service(FlightsServer::new(FlightsApp::new(db_pool.clone())))
}
