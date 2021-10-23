//! The tests in this module are mostly ignored.
//! This is because they are testing against an
//! actual running Traduora instance and will also
//! mess up that instance.

use traduora::{
    auth::{Authenticated, Unauthenticated},
    Client, Login, TraduoraBuilder,
};

pub const HOST: &str = "localhost:8080";
pub const MAIL: &str = "test@test.test";
pub const PASSWORD: &str = "12345678";
pub const USE_HTTP: bool = true;
pub const VALIDATE_CERTS: bool = false;

pub fn build_test_client() -> impl Client<AccessLevel = Unauthenticated> {
    TraduoraBuilder::new(HOST)
        .use_http(USE_HTTP)
        .validate_certs(VALIDATE_CERTS)
        .build()
        .unwrap()
}

pub fn build_auth_test_client() -> impl Client<AccessLevel = Authenticated> {
    TraduoraBuilder::new(HOST)
        .use_http(USE_HTTP)
        .validate_certs(VALIDATE_CERTS)
        .authenticate(Login::password(MAIL, PASSWORD))
        .build()
        .unwrap()
}

mod auth;
mod terms;
mod users;
