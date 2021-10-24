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

/// precondition: project exists.
#[ignore]
#[test]
fn get_project() {
    let client = build_auth_test_client();
    let project = ShowProject("4b915f76-7c81-45a1-b720-b365d271421d".into())
        .query(&client)
        .unwrap();

    println!("{:#?}", project);
    assert_eq!(project.name, PROJECT_NAME);
    assert_eq!(project.description, DESCRIPTION);
}

/// precondition: project exists.
#[ignore]
#[test]
fn patch_project() {
    let new_name = PROJECT_NAME.to_uppercase();
    let new_description = DESCRIPTION.to_uppercase();
    let client = build_auth_test_client();
    let endpoint = EditProject::new(
        "4b915f76-7c81-45a1-b720-b365d271421d".into(),
        new_name.clone(),
        new_description.clone(),
    );
    let project = endpoint.query(&client).unwrap();

    println!("{:#?}", project);
    assert_eq!(project.name, new_name);
    assert_eq!(project.description, new_description);
    assert_eq!(project.id.value(), "4b915f76-7c81-45a1-b720-b365d271421d");
}

/// precondition: project exists.
#[ignore]
#[test]
fn delete_project() {
    let client = build_auth_test_client();
    let endpoint = DeleteProject("4b915f76-7c81-45a1-b720-b365d271421d".into());
    endpoint.query(&client).unwrap();
}
