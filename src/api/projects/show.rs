use http::Method;

use super::Project;
use crate::{api::ProjectId, auth::Authenticated, query::DefaultModel, Endpoint};

/// Get a project by id.
///
/// **Endpoint** `GET /api/v1/projects/{projectId}`
///
/// **Default model** [`Project`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::{projects::ShowProject, Role}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project = ShowProject("b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into()).query(&client)?;
///
/// assert_eq!(project.name, "Traduora API bindings");
/// assert_eq!(project.description, "Translations for this Traduora API bindings rust crate.");
/// assert_eq!(project.terms_count, 5812);
/// assert_eq!(project.locales_count, 2);
/// assert_eq!(project.role, Role::Admin);
/// assert_eq!(project.date.created, Utc.ymd(2021, 10, 23).and_hms_milli(16, 07, 39, 946));
/// assert_eq!(project.date.modified, Utc.ymd(2021, 10, 23).and_hms_milli(19, 24, 34, 000));
/// assert_eq!(project.id.value(), "b1001dd9-e1c0-4fb0-a60d-eaaec304d332");
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct ShowProject(pub ProjectId);

impl Endpoint for ShowProject {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}", self.0).into()
    }
}

impl DefaultModel for ShowProject {
    type Model = Project;
}
