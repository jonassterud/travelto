use serde::Serialize;

/// Flight
#[derive(Debug, Serialize)]
pub struct Flight {
    /// "Airport name, city name, country name" to depart from.
    pub from: String,
    /// "Airport name, city name, country name" to arrive to.
    pub to: String,
    /// Departure date.
    pub departure_date: String,
    /// Arrival date.
    pub return_date: String,
}
