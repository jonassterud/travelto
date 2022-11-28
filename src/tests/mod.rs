use crate::*;

#[test]
fn test_api_get_flights() {
    let config = api::Config::new("NO", "DE", ("24/12/2022", "25/12/2022"), None).unwrap();
    dbg!(api::get_flights(&config).unwrap());
}

#[test]
fn test_kiwi_api_locations_query() {
    let keys = api::Keys::from_env().unwrap();
    let config = kiwi_api::LocationsQueryParams::new(keys.get_kiwi_search_key(), "NO");
    kiwi_api::locations_query(&config).unwrap();
}

#[test]
fn test_kiwi_api_search() {
    let config = api::Config::new("NO", "DE", ("24/12/2022", "25/12/2022"), None).unwrap();
    kiwi_api::search(&config).unwrap();
}
