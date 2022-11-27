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
    pub return_date: Option<String>,
    /// Link to booking site.
    pub link: String,
}

/// Transform a vector of Kiwi flights into a vector of api::Flight's.
impl From<crate::kiwi_api::SearchResponse> for Vec<Flight> {
    fn from(val: crate::kiwi_api::SearchResponse) -> Self {
        val.data
            .iter()
            .map(|data_val| Flight {
                from: data_val.fly_from.to_owned(),
                to: data_val.fly_to.to_owned(),
                departure_date: data_val.local_departure.to_owned(),
                return_date: None, // for now..
                link: data_val.deep_link.to_owned(),
            })
            .collect()
    }
}
