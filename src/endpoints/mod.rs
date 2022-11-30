//! Endpoints for Tide server.

mod index;
mod locations;
mod search;

pub use index::index;
pub use locations::locations;
pub use search::search;
