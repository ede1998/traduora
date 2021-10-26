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

#[ignore]
#[test]
fn get_project_locales() {
    let client = build_auth_test_client();
    let project = "cb6d5506-1762-49f7-9b4d-e4741e30f75d".into();
    let locales = Locales(project).query(&client).unwrap();

    assert_eq!(locales.len(), 2);
    assert_eq!(locales[0].locale.code, "de_DE".into());
    assert_eq!(
        locales[0].id.value(),
        "91ea2d61-8e14-481f-af19-9ae16bbf95a3"
    );

    assert_eq!(locales[1].locale.code, "en".into());
    assert_eq!(
        locales[1].id.value(),
        "38c02384-9be0-4308-aac6-056f1d7b0a08"
    );
}

#[ignore]
#[test]
fn post_project_locale() {
    let client = build_auth_test_client();
    let project = "cb6d5506-1762-49f7-9b4d-e4741e30f75d".into();
    let locale = CreateLocale::new(project, "en_US".into())
        .query(&client)
        .unwrap();

    assert_eq!(locale.locale.code.value(), "en_US");
}
