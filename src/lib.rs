use db::Database;
use rabbitmq::Rabbit;
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
mod errors;
pub mod flights;
pub mod planes;
pub mod proto;
pub mod rabbitmq;

pub fn build_services(db_pool: PgPool, rabbitmq: Rabbit) -> Routes {
    let db = Database::from_pool(db_pool);

    Routes::default()
        .add_service(PlanesServer::new(PlanesApp::new(db.clone())))
        .add_service(AirportsServer::new(AirportsApp::new(db.clone())))
        .add_service(FlightsServer::new(FlightsApp::new(db.clone(), rabbitmq)))
}
