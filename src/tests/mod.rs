use crate::*;

#[test]
fn test_api_get_flights() {
    let config =
        api::Config::new("NO", "DE", ("24/12/2022", "25/12/2022"), ("", ""), 1, 0, 0).unwrap();
    api::get_flights(config).unwrap();
}

#[test]
fn test_api_get_locations() {
    let params = kiwi_api::LocationsQueryParams::new("PRG").unwrap();
    api::get_locations(params).unwrap();
}

#[test]
fn test_kiwi_api_locations_query() {
    let params = kiwi_api::LocationsQueryParams::new("PRG").unwrap();
    kiwi_api::locations_query(params).unwrap();
}

#[test]
fn test_kiwi_api_search() {
    let params =
        kiwi_api::SearchParams::new("NO", "DE", "24/12/2022", "25/12/2022", "", "", 1, 0, 0)
            .unwrap();
    kiwi_api::search(params).unwrap();
}
