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

#[ignore]
#[test]
fn patch_translation() {
    let client = build_auth_test_client();
    let project = "cb6d5506-1762-49f7-9b4d-e4741e30f75d".into();
    let locale = "de_DE".into();
    let term_id = "7eafe83d-1448-49ea-8ae0-f8753cbd669c".into();
    let translation = EditTranslation::new(project, locale, term_id, "new translation")
        .query(&client)
        .unwrap();

    println!("{:#?}", translation);
    assert_eq!(translation.value, "new translation");
    assert!(translation.labels.is_empty());
    assert_eq!(
        translation.term_id.value(),
        "7eafe83d-1448-49ea-8ae0-f8753cbd669c"
    );
}

#[ignore]
#[test]
fn delete_locales() {
    let client = build_auth_test_client();
    let project = "cb6d5506-1762-49f7-9b4d-e4741e30f75d".into();
    let locale = "de_DE".into();
    DeleteLocale::new(project, locale).query(&client).unwrap();
}
