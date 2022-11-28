use serde::Serialize;

/// Location
#[derive(Debug, Serialize)]
pub struct Location {
    /// Location code.
    pub code: String,
    /// Location name.
    pub name: String,
}

/// Transform a Kiwi locations response into a vector of api::Location's.
impl From<crate::kiwi_api::LocationsResponse> for Vec<Location> {
    fn from(val: crate::kiwi_api::LocationsResponse) -> Self {
        val.locations
            .iter()
            .map(|data_val| Location {
                code: data_val.code.to_owned(),
                name: data_val.name.to_owned(),
            })
            .collect()
    }
}
