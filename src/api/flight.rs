use anyhow::Result;
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

impl TryFrom<crate::kiwi_api::SearchResponse> for Vec<Flight> {
    type Error = anyhow::Error;

    fn try_from(val: crate::kiwi_api::SearchResponse) -> Result<Self> {
        fn format_duration(seconds: u64) -> String {
            format!("{}h {}m", (seconds / 60) / 60, (seconds / 60) % 60)
        }

        fn format_date(date: &str) -> Result<String> {
            Ok(date
                .parse::<chrono::DateTime<chrono::Utc>>()?
                .format("%d/%m, %R")
                .to_string())
        }

        val.data
            .iter()
            .map(|x| -> Result<Flight> {
                Ok(Flight {
                    from: format!("{}, {}", x.city_from, x.fly_from),
                    to: format!("{}, {}", x.city_to, x.fly_to),
                    departure_date: format_date(&x.local_departure)?,
                    departure_arrival_date: format_date(&x.local_arrival)?,
                    departure_duration: format_duration(x.duration.departure_secs),
                    return_date: x
                        .route
                        .iter()
                        .find(|r| r.is_return > 0)
                        .map(|r| format_date(&r.local_departure))
                        .transpose()?,
                    return_arrival_date: x
                        .route
                        .iter()
                        .rfind(|r| r.is_return > 0)
                        .map(|r| format_date(&r.local_arrival))
                        .transpose()?, // assumes last route in vector is final
                    return_duration: Some(format_duration(x.duration.return_secs)),
                    price: x.price,
                    link: x.deep_link.to_owned(),
                })
            })
            .collect()
    }
}
