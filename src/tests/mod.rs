use crate::*;

/// Test [`api::get_flights`].
#[ignore = "avoid spam"]
#[test]
fn test_api_get_flights() {
    let config = api::SearchConfig::new(
        "MUC".to_owned(),
        "BER".to_owned(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 24).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 25),
        None,
        None,
        1,
        0,
        0,
    )
    .unwrap();
    api::get_flights(config).unwrap();
}

/// Test [`api::get_locations`].
#[ignore = "avoid spam"]
#[test]
fn test_api_get_locations() {
    let config = api::LocationConfig::new("NO").unwrap();
    api::get_locations(config).unwrap();
}

/// Test [`kiwi_api::locations_query`].
#[ignore = "avoid spam"]
#[test]
fn test_kiwi_api_locations_query() {
    let params = kiwi_api::LocationsQueryParams::new("PRG").unwrap();
    kiwi_api::locations_query(params).unwrap();
}

/// Test [`kiwi_api::search`].
#[ignore = "avoid spam"]
#[test]
fn test_kiwi_api_search() {
    let params = kiwi_api::SearchParams::new(
        "NO".to_owned(),
        "DE".to_owned(),
        "24/12/2022".to_owned(),
        Some("25/12/2022".to_owned()),
        None,
        None,
        1,
        0,
        0,
    )
    .unwrap();
    kiwi_api::search(params).unwrap();
}

/// Test [`skyscanner_api::search`].
#[ignore = "avoid spam"]
#[test]
fn test_skyscanner_api_search() {
    let params = skyscanner_api::SearchParams::new(
        1,
        "MUC".to_owned(),
        "BER".to_owned(),
        "2022-12-24".to_owned(),
        Some("2022-12-24".to_owned()),
    )
    .unwrap();
    skyscanner_api::search(params).unwrap();
}
