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

impl From<crate::kiwi_api::LocationsResponse> for Vec<Location> {
    fn from(val: crate::kiwi_api::LocationsResponse) -> Self {
        val.locations.iter().map(|x| x.into()).collect()
    }
}

impl From<&crate::kiwi_api::Location> for Location {
    fn from(val: &crate::kiwi_api::Location) -> Self {
        Self {
            id: val.id.to_owned(),
            code: val.code.to_owned(),
            name: val.name.to_owned(),
            country: val.country.as_ref().map_or(
                val.city.as_ref().map(|val_city| (&val_city.country).into()),
                |x| Some(x.into()),
            ),
            city: val.city.as_ref().map(|x| x.into()),
            variant: val.variant.to_owned(),
        }
    }
}

impl From<&crate::kiwi_api::LocationCountry> for Country {
    fn from(val: &crate::kiwi_api::LocationCountry) -> Self {
        Self {
            id: val.id.to_owned(),
            name: val.name.to_owned(),
            code: val.code.to_owned(),
        }
    }
}

impl From<&crate::kiwi_api::LocationCity> for City {
    fn from(val: &crate::kiwi_api::LocationCity) -> Self {
        Self {
            id: val.id.to_owned(),
            name: val.name.to_owned(),
            code: val.code.to_owned(),
        }
    }
}
