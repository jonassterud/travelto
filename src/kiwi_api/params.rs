/// Params for locations.
#[derive(Debug)]
pub struct LocationsParams {}

/// Params for search.
#[derive(Debug)]
pub struct SearchParams {
    apikey: String,
    fly_from: String,
    fly_to: String,
    date_from: String,
    date_to: String,
    return_from: Option<String>,
    return_to: Option<String>,
}

impl From<super::api::Config> for SearchParams {
    fn from(val: super::api::Config) -> Self {
        Self {
            apikey: val.keys.get_kiwi_search_key().to_string(),
            fly_from: val.from,
            fly_to: val.to,
            date_from: val.departure_date.0,
            date_to: val.departure_date.1,
            return_from: val.return_date.as_ref().map(|x| x.0.to_owned()),
            return_to: val.return_date.as_ref().map(|x| x.1.to_owned()),
        }
    }
}

impl SearchParams {
    /// Get parameters as a single URL compatible string.
    pub fn as_url_params(&self) -> String {
        // TODO: Probably a better way to do this with macros.
        format!(
            "fly_from={}&fly_to={}&date_from={}&date_to={}&return_from={}&return_to={}",
            self.fly_from,
            self.fly_to,
            self.date_from,
            self.date_to,
            self.return_from.as_ref().unwrap_or(&String::new()),
            self.return_to.as_ref().unwrap_or(&String::new()),
        )
    }

    pub fn get_apikey(&self) -> &str {
        &self.apikey
    }
}
