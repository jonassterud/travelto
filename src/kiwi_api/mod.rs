//! Kiwi search API.

use crate::api;

mod params;
mod response;
pub use params::*;
pub use response::*;

use anyhow::Result;

const BASE_URL: &str = "https://api.tequila.kiwi.com";

/// Kiwi.com locations query API.
///
/// [Resource](https://tequila.kiwi.com/portal/docs/tequila_api/locations_api)
pub fn locations_query(
    params: &(impl Into<LocationsQueryParams> + Clone),
) -> Result<LocationsResponse> {
    let params: LocationsQueryParams = params.clone().into();
    let path = format!("{}/locations/query?{}", BASE_URL, params.as_url_params());

    let resp = ureq::get(&path)
        .set("apikey", params.get_apikey())
        .call()?
        .into_json()?;

    Ok(resp)
}

/// Kiwi.com search API.
///
/// [Resource](https://tequila.kiwi.com/portal/docs/tequila_api/search_api)
pub fn search(params: &(impl Into<SearchParams> + Clone)) -> Result<SearchResponse> {
    let params: SearchParams = params.clone().into();
    let path = format!("{}/v2/search?{}", BASE_URL, params.as_url_params());

    let resp = ureq::get(&path)
        .set("apikey", params.get_apikey())
        .call()?
        .into_json()?;

    Ok(resp)
}
