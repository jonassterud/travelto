//! Endpoints for Tide server.

#![deny(missing_docs)]

mod index;
mod locations;
mod search;

pub use index::*;
pub use locations::*;
pub use search::*;
