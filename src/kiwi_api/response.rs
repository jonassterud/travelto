//! API responses as structs.

use serde::Deserialize;

/// Response struct for locations.
#[derive(Debug, Deserialize)]
pub struct LocationsResponse {
    /// Vector of locations.
    pub locations: Vec<LocationsResponseLocation>,
}

/// Location.
#[derive(Debug, Deserialize)]
pub struct LocationsResponseLocation {
    /// Location id.
    pub id: String,
    /// Location name.
    pub name: String,
}

/// Response struct for search.
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    /// Vector of search responses.
    pub data: Vec<SearchResponseData>,
}

/// Search response.
#[derive(Debug, Deserialize)]
pub struct SearchResponseData {
    /// Deep link?
    pub deep_link: String,
    /// Duration of flight.
    pub duration: SearchResponseDataDuration,
    /// Local arrival time.
    pub local_arrival: String,
    /// Local departure time.
    pub local_departure: String,
    /// Price of flight.
    pub price: u32,
    /// Vector of routes.
    pub route: Vec<SearchResponseDataRoute>,
    /// Airport to fly to.
    #[serde(rename = "flyTo")]
    pub fly_to: String,
    /// Airport to fly from.
    #[serde(rename = "flyFrom")]
    pub fly_from: String,
    /// City to fly from.
    #[serde(rename = "cityFrom")]
    pub city_from: String,
    /// City to fly to.
    #[serde(rename = "cityTo")]
    pub city_to: String,
}

/// Duration.
#[derive(Debug, Deserialize)]
pub struct SearchResponseDataDuration {
    /// Departure time in seconds.
    #[serde(rename = "departure")]
    pub departure_secs: u64,
    /// Return time in seconds.
    #[serde(rename = "return")]
    pub return_secs: u64,
    /// Total time in seconds.
    #[serde(rename = "total")]
    pub total_secs: u64,
}

/// Route.
#[derive(Debug, Deserialize)]
pub struct SearchResponseDataRoute {
    /// Local arrival time.
    pub local_arrival: String,
    /// Local departure time.
    pub local_departure: String,
    #[serde(rename = "return")]
    /// Boolean (as u32) showing if route is part of return journey.
    pub is_return: u32,
    /// Airport to fly to.
    #[serde(rename = "flyTo")]
    pub fly_to: String,
    /// Airport to fly from.
    #[serde(rename = "flyFrom")]
    pub fly_from: String,
}
