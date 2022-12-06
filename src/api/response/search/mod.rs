mod flight;

use flight::Flight;
use serde::Serialize;

/// Search response.
#[derive(Debug, Default, Serialize)]
pub struct SearchResponse {
    /// Vector of flights.
    pub flights: Vec<Flight>,
    /// Boolean showing if all search is finished.
    pub finished: bool,
}

impl SearchResponse {
    /// Create a new [`SearchResponse`].
    pub fn new(flights: Vec<Flight>, finished: bool) -> SearchResponse {
        SearchResponse { flights, finished }
    }
}
