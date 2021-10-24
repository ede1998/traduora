use super::*;
use traduora::{
    api::{projects::*, Role},
    Query,
};

const PROJECT_NAME: &str = "Traduora API bindings";
const DESCRIPTION: &str = "Translations for this Traduora API bindings rust crate.";

/// precondition: default project exists.
#[ignore]
#[test]
fn get_projects() {
    let client = build_auth_test_client();
    let projects = Projects.query(&client).unwrap();
    println!("{:#?}", projects);
    assert!(!projects.is_empty());
    assert_eq!(projects[0].name, PROJECT_NAME);
    assert_eq!(projects[0].description, DESCRIPTION);
}

/// precondition: default user exists.
#[ignore]
#[test]
fn post_create_project() {
    let client = build_auth_test_client();
    let endpoint = CreateProject::new(PROJECT_NAME, DESCRIPTION);
    let project = endpoint.query(&client).unwrap();

    println!("{:#?}", project);
    assert_eq!(project.name, PROJECT_NAME);
    assert_eq!(project.description, DESCRIPTION);
    assert_eq!(project.role, Role::Admin);
    assert_eq!(project.locales_count, 0);
    assert_eq!(project.terms_count, 0);
}
