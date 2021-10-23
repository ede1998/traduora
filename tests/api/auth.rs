use super::*;
use traduora::{api::auth::*, Query};

/// precondition: none.
#[ignore]
#[test]
fn get_providers() {
    let client = build_test_client();
    let providers = Providers.query(&client).unwrap();
    println!("Providers: {:?}", providers);
    assert_eq!(0, providers.len());
}

/// precondition: no default user exists.
#[ignore]
#[test]
fn signup() {
    let client = build_test_client();
    let signup = Signup::new("Tester", super::MAIL, super::PASSWORD);
    let new_user = signup.query(&client).unwrap();
    assert_eq!(super::MAIL, new_user.email);
    assert_eq!("Tester", new_user.name);
}

/// precondition: default user exists.
#[ignore]
#[test]
fn token() {
    let client = build_test_client();
    let token_request = Token::password(super::MAIL, super::PASSWORD);
    let token = token_request.query(&client).unwrap();
    assert_ne!(0, token.expires_in.len());
    assert_eq!("bearer", token.token_type.to_lowercase());
}
