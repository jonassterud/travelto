use serde::Serialize;

/// Location.
#[derive(Debug, Serialize)]
pub struct Location {
    /// Location id.
    pub id: String,
    /// Location name.
    pub name: String,
}

impl From<crate::kiwi_api::LocationsResponse> for Vec<Location> {
    fn from(val: crate::kiwi_api::LocationsResponse) -> Self {
        val.locations
            .iter()
            .map(|data_val| Location {
                id: data_val.id.to_owned(),
                name: data_val.name.to_owned(),
            })
            .collect()
    }
}
