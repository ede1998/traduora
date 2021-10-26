use super::*;
use traduora::{api::locales::*, Query};

/// precondition: default user exists
#[ignore]
#[test]
fn get_locales() {
    let client = build_auth_test_client();
    let locales = AllLocales.query(&client).unwrap();

    println!("{:#?}", locales);

    assert!(locales.len() > 500);
    let english = locales.iter().find(|l| l.code.value() == "en_US").unwrap();
    assert_eq!(english.language, "English");
    assert_eq!(english.region, "United States");
}
