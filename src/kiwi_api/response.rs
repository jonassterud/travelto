//! API responses as structs.

use serde::Deserialize;

/// Response struct for locations.
#[derive(Debug, Deserialize)]
pub struct LocationsResponse {
    pub locations: Vec<LocationsResponseLocation>,
}

#[derive(Debug, Deserialize)]
pub struct LocationsResponseLocation {
    pub code: String,
    pub name: String,
}

/// Response struct for search.
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub data: Vec<SearchResponseData>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponseData {
    pub deep_link: String,
    pub duration: SearchResponseDataDuration,
    pub local_arrival: String,
    pub local_departure: String,
    pub price: u32,
    #[serde(rename = "flyTo")]
    pub fly_to: String,
    #[serde(rename = "flyFrom")]
    pub fly_from: String,
    #[serde(rename = "cityFrom")]
    pub city_from: String,
    #[serde(rename = "cityTo")]
    pub city_to: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponseDataDuration {
    #[serde(rename = "departure")]
    pub departure_secs: u32,
    #[serde(rename = "return")]
    pub return_secs: u32,
    #[serde(rename = "total")]
    pub total_secs: u32,
}
