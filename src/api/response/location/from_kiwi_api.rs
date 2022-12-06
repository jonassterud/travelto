impl From<crate::kiwi_api::LocationsResponse> for Vec<super::Location> {
    fn from(val: crate::kiwi_api::LocationsResponse) -> Self {
        val.locations.iter().map(|x| x.into()).collect()
    }
}

impl From<&crate::kiwi_api::Location> for super::Location {
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

impl From<&crate::kiwi_api::LocationCountry> for super::Country {
    fn from(val: &crate::kiwi_api::LocationCountry) -> Self {
        Self {
            id: val.id.to_owned(),
            name: val.name.to_owned(),
            code: val.code.to_owned(),
        }
    }
}

impl From<&crate::kiwi_api::LocationCity> for super::City {
    fn from(val: &crate::kiwi_api::LocationCity) -> Self {
        Self {
            id: val.id.to_owned(),
            name: val.name.to_owned(),
            code: val.code.to_owned(),
        }
    }
}
