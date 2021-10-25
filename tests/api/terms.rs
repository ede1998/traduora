use super::*;
use traduora::{
    api::{terms::*, ProjectId},
    Query,
};

/// precondition: default user exists and has access to project
/// b1001dd9-e1c0-4fb0-a60d-eaaec304d332
#[ignore]
#[test]
fn post_term() {
    let client = build_auth_test_client();
    let term = CreateTerm::new(
        "hello.world",
        ProjectId::new("b1001dd9-e1c0-4fb0-a60d-eaaec304d332"),
    );
    let new_term = term.query(&client).unwrap();
    assert!(new_term.labels.is_empty());
    assert_eq!("hello.world", new_term.value);
}

/// precondition: default user exists and has access to project
#[ignore]
#[test]
fn get_terms() {
    let client = build_auth_test_client();
    let terms = Terms("b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into())
        .query(&client)
        .unwrap();

    println!("{:#?}", terms);
    assert!(terms.len() > 1);
    assert_eq!(terms[0].value, "hello.world");
    assert!(terms[0].labels.is_empty());
}

/// precondition: term and project exist.
#[ignore]
#[test]
fn patch_term() {
    let client = build_auth_test_client();
    let project = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
    let term = "0fa39756-65db-423c-a6d9-534b62fe9ead".into();
    let edited_term = EditTerm::new(project, term, "new.term.text.1")
        .query(&client)
        .unwrap();

    println!("{:#?}", edited_term);
    assert_eq!(edited_term.value, "new.term.text.1");
}
