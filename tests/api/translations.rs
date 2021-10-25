use super::*;
use traduora::{api::translations::*, Query};

/// precondition: project and locale exist.
#[ignore]
#[test]
fn get_translations() {
    let client = build_auth_test_client();
    let project = "cb6d5506-1762-49f7-9b4d-e4741e30f75d".into();
    let locale = "de_DE".into();
    let translations = Translations::new(project, locale).query(&client).unwrap();

    println!("{:#?}", translations);

    assert!(!translations.is_empty());
}
