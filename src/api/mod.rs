//! Main API.

use crate::kiwi_api;

mod config;
mod flight;
mod state;
pub use config::Config;
pub use flight::Flight;
pub use state::State;

use anyhow::Result;

pub fn get_flights(config: &Config) -> Result<Vec<Flight>> {
    let mut out = vec![];

    out.append(&mut kiwi_api::search(config)?.into());
    // ...

    Ok(out)
}
