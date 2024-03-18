use super::queries;
use proto;

impl From<queries::Airport> for proto::flightmngr::Airport {
    fn from(airport: queries::Airport) -> Self {
        Self {
            id: airport.id.to_string(),
            icao: airport.icao,
            iata: airport.iata,
            name: airport.name,
            country: airport.country,
            city: airport.city,
            deleted: airport.deleted,
        }
    }
}
