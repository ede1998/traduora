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
