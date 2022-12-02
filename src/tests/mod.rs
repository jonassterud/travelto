use crate::*;

/// Test [`api::get_flights`].
#[test]
fn test_api_get_flights() {
    let config =
        api::SearchConfig::new("NO", "DE", ("24/12/2022", "25/12/2022"), ("", ""), 1, 0, 0)
            .unwrap();
    api::get_flights(config).unwrap();
}

/// Test [`api::get_locations`].
#[test]
fn test_api_get_locations() {
    let config = api::LocationConfig::new("NO").unwrap();
    api::get_locations(config).unwrap();
}

/// Test [`kiwi_api::locations_query`].
#[test]
fn test_kiwi_api_locations_query() {
    let params = kiwi_api::LocationsQueryParams::new("PRG").unwrap();
    kiwi_api::locations_query(params).unwrap();
}

/// Test [`kiwi_api::search`].
#[test]
fn test_kiwi_api_search() {
    let params =
        kiwi_api::SearchParams::new("NO", "DE", "24/12/2022", "25/12/2022", "", "", 1, 0, 0)
            .unwrap();
    kiwi_api::search(params).unwrap();
}
