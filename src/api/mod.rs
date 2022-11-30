//! Main API.

use crate::kiwi_api;

mod config;
mod flight;
mod location;
mod state;
pub use config::{Keys, LocationConfig, SearchConfig};
pub use flight::Flight;
pub use location::Location;
pub use state::State;

use anyhow::Result;

/// Combines flight results from different APIs into a single vector.
pub fn get_flights(config: SearchConfig) -> Result<Vec<Flight>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::search(config.into())?.into());
    // ...

    Ok(out)
}

/// Search trough locations by using the Kiwi.com locations API.
///
/// [Resource](https://tequila.kiwi.com/portal/docs/tequila_api/locations_api)
pub fn get_locations(config: LocationConfig) -> Result<Vec<Location>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::locations_query(config.into())?.into());

    Ok(out)
}
