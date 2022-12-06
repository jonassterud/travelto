mod from_kiwi_api;

use serde::Serialize;

/// Location.
#[derive(Debug, Serialize)]
pub struct Location {
    /// Location id.
    pub id: String,
    /// Location code.
    pub code: String,
    /// Location name.
    pub name: String,
    /// Location country.
    pub country: Option<Country>,
    /// Location city.
    pub city: Option<City>,
    /// Location type.
    pub variant: String,
}

/// Country.
#[derive(Debug, Serialize)]
pub struct Country {
    /// Country id.
    pub id: String,
    /// Country name.
    pub name: String,
    /// Country code.
    pub code: String,
}

/// City.
#[derive(Debug, Serialize)]
pub struct City {
    /// City id.
    pub id: String,
    /// City name.
    pub name: String,
    /// City code.
    pub code: String,
}
