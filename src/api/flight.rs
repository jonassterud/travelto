use humantime::format_duration;
use serde::Serialize;
use std::time::Duration;

/// Flight
#[derive(Debug, Serialize)]
pub struct Flight {
    /// "City name, airport code" to depart from.
    pub from: String,
    /// "City name, airport code" to arrive to.
    pub to: String,
    /// Departure date.
    pub departure_date: String,
    /// Duration of flight departure.
    pub departure_duration: String,
    /// Arrival date.
    pub return_date: Option<String>,
    /// Duration of flight return.
    pub return_duration: String,
    /// Price of flight.
    pub price: u32,
    /// Link to booking site.
    pub link: String,
}

/// Transform a Kiwi flights response into a vector of api::Flight's.
impl From<crate::kiwi_api::SearchResponse> for Vec<Flight> {
    fn from(val: crate::kiwi_api::SearchResponse) -> Self {
        val.data
            .iter()
            .map(|data_val| Flight {
                from: format!("{}, {}", data_val.city_from, data_val.fly_from),
                to: format!("{}, {}", data_val.city_to, data_val.fly_to),
                departure_date: data_val.local_departure.to_owned(),
                departure_duration: format_duration(Duration::from_secs(
                    data_val.duration.departure_secs,
                ))
                .to_string(),
                return_date: None, // for now..
                return_duration: format_duration(Duration::from_secs(
                    data_val.duration.return_secs,
                ))
                .to_string(),
                price: data_val.price,
                link: data_val.deep_link.to_owned(),
            })
            .collect()
    }
}
