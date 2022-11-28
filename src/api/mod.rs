//! Main API.

use crate::kiwi_api;

mod config;
mod flight;
mod location;
mod state;
pub use config::{Config, Keys};
pub use flight::Flight;
pub use location::Location;
pub use state::State;

use anyhow::Result;

/// Combines flight results from different APIs into a single vector.
pub fn get_flights(config: &Config) -> Result<Vec<Flight>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::search(config)?.into());
    // ...

    Ok(out)
}

/// Search trough locations by using the Kiwi.com locations API.
///
/// [Resource](https://tequila.kiwi.com/portal/docs/tequila_api/locations_api)
pub fn get_locations(
    params: &(impl Into<kiwi_api::LocationsQueryParams> + Clone),
) -> Result<Vec<Location>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::locations_query(params)?.into());

    Ok(out)
}
