//! Kiwi search API.

use crate::api;

mod params;
mod response;
pub use params::*;
pub use response::*;

use anyhow::Result;

const BASE_URL: &str = "https://api.tequila.kiwi.com/v2";

pub fn locations() -> Result<LocationsResponse> {
    todo!()
}

pub fn search(params: &(impl Into<SearchParams> + Clone)) -> Result<SearchResponse> {
    let params: SearchParams = params.clone().into();
    let path = format!("{}/search?{}", BASE_URL, params.as_url_params());

    let resp = ureq::get(&path)
        .set("apikey", params.get_apikey())
        .call()?
        .into_json()?;

    Ok(resp)
}
