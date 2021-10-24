use super::*;
use traduora::{api::projects::*, Query};

const PROJECT_NAME: &str = "Traduora API bindings";
const DESCRIPTION: &str = "Translations for this Traduora API bindings rust crate.";

/// precondition: default project exists.
#[ignore]
#[test]
fn get_me() {
    let client = build_auth_test_client();
    let projects = Projects.query(&client).unwrap();
    println!("{:#?}", projects);
    assert!(!projects.is_empty());
    assert_eq!(projects[0].name, PROJECT_NAME);
    assert_eq!(projects[0].description, DESCRIPTION);
}
