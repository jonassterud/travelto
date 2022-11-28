/// Params for locations query.
#[derive(Debug, Clone)]
pub struct LocationsQueryParams {
    apikey: String,
    term: String,
}

impl LocationsQueryParams {
    /// Create a new [`LocationsQueryParams`].
    pub fn new(apikey: &str, term: &str) -> LocationsQueryParams {
        LocationsQueryParams {
            apikey: apikey.to_owned(),
            term: term.to_owned(),
        }
    }

    /// Get parameters as a single URL compatible string.
    pub fn as_url_params(&self) -> String {
        format!("term={}", self.term,)
    }

    /// Get api key.
    pub fn get_apikey(&self) -> &str {
        &self.apikey
    }
}

/// Params for search.
#[derive(Debug)]
pub struct SearchParams {
    apikey: String,
    fly_from: String,
    fly_to: String,
    date_from: String,
    date_to: String,
    return_from: String,
    return_to: String,
    adults: u32,
    children: u32,
    infants: u32,
}

impl From<super::api::Config> for SearchParams {
    fn from(val: super::api::Config) -> Self {
        Self {
            apikey: val.keys.get_kiwi_search_key().to_string(),
            fly_from: val.from,
            fly_to: val.to,
            date_from: val.departure_date.0,
            date_to: val.departure_date.1,
            return_from: val.return_date.0,
            return_to: val.return_date.1,
            adults: val.adults,
            children: val.children,
            infants: val.infants,
        }
    }
}

impl SearchParams {
    /// Get parameters as a single URL compatible string.
    pub fn as_url_params(&self) -> String {
        // TODO: Probably a better way to do this with macros.
        format!(
            "fly_from={}&fly_to={}&date_from={}&date_to={}&return_from={}&return_to={}&adults={}&children={}&infants={}",
            self.fly_from,
            self.fly_to,
            self.date_from,
            self.date_to,
            self.return_from,
            self.return_to,
            self.adults,
            self.children,
            self.infants,
        )
    }

    /// Get api key.
    pub fn get_apikey(&self) -> &str {
        &self.apikey
    }
}
