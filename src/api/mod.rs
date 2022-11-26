//! Main API

mod config;
mod flight;
mod state;
pub use config::Config;
pub use flight::Flight;
pub use state::State;

use anyhow::Result;

pub fn get_flights() -> Result<Vec<Flight>> {
    todo!()
}
