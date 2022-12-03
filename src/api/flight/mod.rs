mod from_kiwi_api;
mod from_skyscanner_api;

use serde::Serialize;

/// Flight
#[derive(Debug, Serialize)]
pub struct Flight {
    /// "City name, airport code" to depart from.
    pub from: String,
    /// "City name, airport code" to arrive to.
    pub to: String,
    /// Departure date.
    pub departure_date: String,
    /// Arrival date.
    pub departure_arrival_date: String,
    /// Duration of flight departure.
    pub departure_duration: String,
    /// Return date.
    pub return_date: Option<String>,
    /// Return arrival date.
    pub return_arrival_date: Option<String>,
    /// Duration of flight return.
    pub return_duration: Option<String>,
    /// Price of flight.
    pub price: u32,
    /// Link to booking site.
    pub link: String,
}
