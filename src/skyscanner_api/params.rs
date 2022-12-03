use crate::api;
use anyhow::Result;

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
    departure_date: String,
    /// Return date.
    return_date: Option<String>,
}

impl From<super::api::SearchConfig> for SearchParams {
    fn from(val: super::api::SearchConfig) -> Self {
        Self {
            key: val.keys.get_rapid_key().to_owned(),
            host: val.keys.get_rapid_skyscanner_host().to_owned(),
            adults: val.adults,
            origin: val.from,                   // oops, must be IATA code
            destination: val.to,                // oops, must be IATA code
            departure_date: val.departure_from.format("%Y-%m-%d").to_string(), // doesn't support range
            return_date: val.return_from.map(|x| x.format("%Y-%m-%d").to_string()),       // doesn't support range
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
        departure_date: String,
        return_date: Option<String>,
    ) -> Result<SearchParams> {
        Ok(SearchParams {
            key: api::Keys::from_env()?.get_rapid_key().to_owned(),
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
            self.departure_date,
            self.return_date.as_ref().unwrap_or(&String::new()),
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
