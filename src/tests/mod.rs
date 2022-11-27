use crate::*;

#[test]
fn test_api_get_flights() {
    let config = api::Config::new("NO", "DE", ("24/12/2022", "25/12/2022"), None).unwrap();
    dbg!(api::get_flights(&config).unwrap());
}

#[test]
fn test_kiwi_api_locations() {}

#[test]
fn test_kiwi_api_search() {
    let config = api::Config::new("NO", "DE", ("24/12/2022", "25/12/2022"), None).unwrap();
    kiwi_api::search(&config).unwrap();
}
