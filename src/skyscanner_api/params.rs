use crate::api;
use anyhow::Result;
use chrono::NaiveDate;

/// Params for search.
#[derive(Debug)]
pub struct SearchParams {
    /// X-RapidAPI-Key
    key: String,
    /// X-RapidAPI-Host
    host: String,
    /// Number of adult passengers (16 and over)
    adults: u32,
    /// Departure airport IATA code
    origin: String,
    /// Destination airport IATA code
    destination: String,
    /// Departure date.
    departure_date: NaiveDate,
    /// Return date.
    return_date: Option<NaiveDate>,
}

impl From<super::api::SearchConfig> for SearchParams {
    fn from(val: super::api::SearchConfig) -> Self {
        Self {
            key: val.keys.get_rapid_skyscanner_key().to_owned(),
            host: val.keys.get_rapid_skyscanner_host().to_owned(),
            adults: val.adults,
            origin: val.from,                   // oops, must be IATA code
            destination: val.to,                // oops, must be IATA code
            departure_date: val.departure_from, // doesn't support range
            return_date: val.return_from,       // doesn't support range
        }
    }
}

impl SearchParams {
    /// Create a new [`SearchParams`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adults: u32,
        origin: String,
        destination: String,
        departure_date: NaiveDate,
        return_date: Option<NaiveDate>,
    ) -> Result<SearchParams> {
        Ok(SearchParams {
            key: api::Keys::from_env()?.get_kiwi_search_key().to_owned(),
            host: api::Keys::from_env()?
                .get_rapid_skyscanner_host()
                .to_owned(),
            adults,
            origin,
            destination,
            departure_date,
            return_date,
        })
    }

    /// Get parameters as a single URL compatible string.
    pub fn as_url_params(&self) -> String {
        // TODO: Probably a better way to do this with macros.
        format!(
            "adults={}&origin={}&destination={}&departureDate={}&returnDate={}",
            self.adults,
            self.origin,
            self.destination,
            self.departure_date.format("%Y-%m-%d"),
            self.return_date
                .map_or(String::new(), |x| x.format("%Y-%m-%d").to_string()),
        )
    }

    /// Get key.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Get host.
    pub fn get_host(&self) -> &str {
        &self.host
    }
}
