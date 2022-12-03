use crate::api;
use anyhow::Result;

/// Params for locations query.
#[derive(Debug, Clone)]
pub struct LocationsQueryParams {
    apikey: String,
    term: String,
}

impl LocationsQueryParams {
    /// Create a new [`LocationsQueryParams`].
    pub fn new(term: &str) -> Result<LocationsQueryParams> {
        Ok(LocationsQueryParams {
            apikey: api::Keys::from_env()?.get_kiwi_search_key().to_owned(),
            term: term.to_owned(),
        })
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

impl From<super::api::LocationConfig> for LocationsQueryParams {
    fn from(val: super::api::LocationConfig) -> Self {
        Self {
            apikey: val.keys.get_kiwi_search_key().to_owned(),
            term: val.term,
        }
    }
}

/// Params for search.
#[derive(Debug)]
pub struct SearchParams {
    apikey: String,
    fly_from: String,
    fly_to: String,
    date_from: String,
    date_to: Option<String>,
    return_from: Option<String>,
    return_to: Option<String>,
    adults: u32,
    children: u32,
    infants: u32,
}

impl From<super::api::SearchConfig> for SearchParams {
    fn from(val: super::api::SearchConfig) -> Self {
        Self {
            apikey: val.keys.get_kiwi_search_key().to_owned(),
            fly_from: val.from,
            fly_to: val.to,
            date_from: val.departure_from,
            date_to: val.departure_to,
            return_from: val.return_from,
            return_to: val.return_to,
            adults: val.adults,
            children: val.children,
            infants: val.infants,
        }
    }
}

impl SearchParams {
    /// Create a new [`SearchParams`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fly_from: String,
        fly_to: String,
        date_from: String,
        date_to: Option<String>,
        return_from: Option<String>,
        return_to: Option<String>,
        adults: u32,
        children: u32,
        infants: u32,
    ) -> Result<SearchParams> {
        Ok(SearchParams {
            apikey: api::Keys::from_env()?.get_kiwi_search_key().to_owned(),
            fly_from,
            fly_to,
            date_from,
            date_to,
            return_from,
            return_to,
            adults,
            children,
            infants,
        })
    }

    /// Get parameters as a single URL compatible string.
    pub fn as_url_params(&self) -> String {
        // TODO: Probably a better way to do this with macros.
        format!(
            "fly_from={}&fly_to={}&date_from={}&date_to={}&return_from={}&return_to={}&adults={}&children={}&infants={}",
            self.fly_from,
            self.fly_to,
            self.date_from,
            self.date_to.as_ref().unwrap_or(&String::new()),
            self.return_from.as_ref().unwrap_or(&String::new()),
            self.return_to.as_ref().unwrap_or(&String::new()),
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
