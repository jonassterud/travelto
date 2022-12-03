//! Skyscanner search API.

#![deny(missing_docs)]

mod params;
mod response;

use crate::api;
use anyhow::Result;
pub use params::*;
pub use response::*;

const DELAY_BETWEEN_REQUESTS: u64 = 15;

/// Skyscanner search API.
///
/// [Read more](https://rapidapi.com/3b-data-3b-data-default/api/skyscanner44).
///
/// # Arguments
///
/// * `params` - [`SearchParams`].
pub fn search(params: SearchParams) -> Result<SearchResponse> {
    let path = format!(
        "https://{}/search?{}",
        params.get_host(),
        params.as_url_params()
    );

    loop {
        let resp = ureq::get(&path)
            .set("X-RapidAPI-Key", params.get_key())
            .set("X-RapidAPI-Host", params.get_host())
            .call()?
            .into_json::<SearchResponse>()?;

        if resp.context.status == "complete" {
            return Ok(resp);
        } else {
            let dur = std::time::Duration::from_secs(DELAY_BETWEEN_REQUESTS);
            std::thread::sleep(dur);
        }
    }
}
