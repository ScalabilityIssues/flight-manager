use super::{data::FlightData, queries};
use crate::{
    datautils::convert_odt_to_timestamp,
    proto::{self, flightmngr::FlightStatusEvent},
};

impl From<FlightData> for proto::flightmngr::Flight {
    fn from(flight_data: FlightData) -> Self {
        let FlightData(flight, cancelled, delayed, gate_dep, gate_arr) = flight_data;

        // extract last event statuses
        let is_cancelled = !cancelled.is_empty();

        let last_delay = delayed.iter().max_by_key(|e| e.timestamp);
        let exp_dep_t = last_delay.map(|e| convert_odt_to_timestamp(e.departure_time));
        let exp_arr_t = last_delay.map(|e| convert_odt_to_timestamp(e.arrival_time));

        let last_gate_dep = gate_dep.iter().max_by_key(|e| e.timestamp);
        let departure_gate = last_gate_dep.map(|e| e.gate.clone());
        let last_gate_arr = gate_arr.iter().max_by_key(|e| e.timestamp);
        let arrival_gate = last_gate_arr.map(|e| e.gate.clone());

        // build history of status events
        let status_events: Vec<FlightStatusEvent> = (cancelled.into_iter().map(Into::into))
            .chain(delayed.into_iter().map(Into::into))
            .chain(gate_dep.into_iter().map(Into::into))
            .chain(gate_arr.into_iter().map(Into::into))
            .collect();

        // assemble
        Self {
            id: flight.id.to_string(),
            plane_id: flight.plane_id.to_string(),
            origin_id: flight.origin_id.to_string(),
            destination_id: flight.destination_id.to_string(),
            departure_time: Some(convert_odt_to_timestamp(flight.departure_time)),
            arrival_time: Some(convert_odt_to_timestamp(flight.arrival_time)),
            status_events,
            // computed fields
            is_cancelled,
            expected_departure_time: exp_dep_t,
            expected_arrival_time: exp_arr_t,
            departure_gate,
            arrival_gate,
        }
    }
}

impl From<queries::EventCancelled> for proto::flightmngr::FlightStatusEvent {
    fn from(event: queries::EventCancelled) -> Self {
        Self {
            timestamp: Some(convert_odt_to_timestamp(event.timestamp)),
            event: Some(
                proto::flightmngr::flight_status_event::Event::FlightCancelled(
                    proto::flightmngr::FlightCancelled {
                        reason: event.reason.unwrap_or_default(),
                    },
                ),
            ),
        }
    }
}

impl From<queries::EventDelayed> for proto::flightmngr::FlightStatusEvent {
    fn from(event: queries::EventDelayed) -> Self {
        Self {
            timestamp: Some(convert_odt_to_timestamp(event.timestamp)),
            event: Some(
                proto::flightmngr::flight_status_event::Event::FlightDelayed(
                    proto::flightmngr::FlightDelayed {
                        arrival_time: Some(convert_odt_to_timestamp(event.arrival_time)),
                        departure_time: Some(convert_odt_to_timestamp(event.departure_time)),
                    },
                ),
            ),
        }
    }
}

impl From<queries::EventGateDepartureSet> for proto::flightmngr::FlightStatusEvent {
    fn from(event: queries::EventGateDepartureSet) -> Self {
        Self {
            timestamp: Some(convert_odt_to_timestamp(event.timestamp)),
            event: Some(
                proto::flightmngr::flight_status_event::Event::FlightGateDeparture(
                    proto::flightmngr::FlightGateDeparture { gate: event.gate },
                ),
            ),
        }
    }
}

impl From<queries::EventGateArrivalSet> for proto::flightmngr::FlightStatusEvent {
    fn from(event: queries::EventGateArrivalSet) -> Self {
        Self {
            timestamp: Some(convert_odt_to_timestamp(event.timestamp)),
            event: Some(
                proto::flightmngr::flight_status_event::Event::FlightGateArrival(
                    proto::flightmngr::FlightGateArrival { gate: event.gate },
                ),
            ),
        }
    }
}
