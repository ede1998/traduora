use http::Method;
use serde::Serialize;

use super::Project;
use crate::{auth::Authenticated, query::DefaultModel, Endpoint};

/// List all projects the current user has any form of access to.
///
/// **Endpoint** `GET /api/v1/projects`
///
/// **Default model** [`Project`]
///
/// # Examples
/// ```
/// # use traduora::{TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::{projects::Projects, Role}, Query};
///
/// let client = Traduora::new("localhost:8080")?;
/// let projects = Projects.query(&client)?;
///
/// assert_eq!(projects.len(), 2);
/// assert_eq!(projects[0].name, "Traduora API bindings");
/// assert_eq!(projects[0].description, "Translations for this Traduora API bindings rust crate.");
/// assert_eq!(projects[0].terms_count, 5812);
/// assert_eq!(projects[0].locales_count, 2);
/// assert_eq!(projects[0].role, Role::Admin);
/// assert_eq!(projects[0].date.created, Utc.ymd(2021, 10, 23).and_hms_milli(16, 07, 39, 946));
/// assert_eq!(projects[0].date.modified, Utc.ymd(2021, 10, 23).and_hms_milli(19, 24, 34, 000));
/// assert_eq!(projects[0].id.value(), "b1001dd9-e1c0-4fb0-a60d-eaaec304d332");
///
/// assert_eq!(projects[1].name, "Test project");
/// assert_eq!(projects[1].description, "Simple project to mess around with.");
/// assert_eq!(projects[1].terms_count, 3);
/// assert_eq!(projects[1].locales_count, 1);
/// assert_eq!(projects[1].role, Role::Viewer);
/// assert_eq!(projects[1].date.created, Utc.ymd(2021, 10, 22).and_hms_milli(12, 45, 13, 138));
/// assert_eq!(projects[1].date.modified, Utc.ymd(2021, 10, 23).and_hms_milli(19, 51, 47, 391));
/// assert_eq!(projects[1].id.value(), "64f92751-ef8f-4d1e-83d1-ea10e6939db9");
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Projects;

impl Endpoint for Projects {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "projects".into()
    }
}

impl DefaultModel for Projects {
    type Model = Vec<Project>;
}
