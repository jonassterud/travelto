//! Main API.

#![deny(missing_docs)]

mod config;
mod flight;
mod location;
mod state;

use crate::{kiwi_api, skyscanner_api};
use anyhow::Result;
pub use config::*;
pub use flight::*;
pub use location::*;
pub use state::*;

/// Combines flight results from different APIs into a single vector.
pub fn get_flights(config: SearchConfig) -> Result<Vec<Flight>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::search(config.clone().into()).map_or(Ok(vec![]), |x| x.try_into())?);
    out.append(&mut skyscanner_api::search(config.into()).map_or(Ok(vec![]), |x| x.try_into())?);

    out.sort_by(|a, b| a.price.cmp(&b.price));

    Ok(out)
}

/// Search trough locations by using the Kiwi.com locations API.
///
/// [Read more](https://tequila.kiwi.com/portal/docs/tequila_api/locations_api).
pub fn get_locations(config: LocationConfig) -> Result<Vec<Location>> {
    let mut out = vec![];
    out.append(&mut kiwi_api::locations_query(config.into())?.into());

    // Filter out certain locations
    out.retain(|x| matches!(x.variant.as_str(), "country" | "city" | "airport"));

    Ok(out)
}
