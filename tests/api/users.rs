use super::*;
use traduora::{api::users::*, Query};

/// precondition: default user exists.
#[ignore]
#[test]
fn get_me() {
    let client = build_auth_test_client();
    let user = Me.query(&client).unwrap();
    assert_eq!(super::MAIL, user.email);
    assert_eq!("Tester", user.name);
}

/// precondition: logged in as user
#[ignore]
#[test]
fn delete_me() {
    let client = build_auth_test_client();
    DeleteMe.query(&client).unwrap();
}
