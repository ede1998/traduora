use http::Method;
use serde::Serialize;

use super::Project;
use crate::{
    api::{self, ProjectId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Update a project's name and description.
///
/// **Endpoint** `PATCH /api/v1/projects/{projectId}`
///
/// **Default model** [`Project`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::{projects::EditProject, Role}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let endpoint = EditProject::new(
///     "1e7dfcea-85ff-4427-9401-aa2bbd99ac80".into(),
///     "Traduora API rust bindings",
///     "Translations for this Traduora API bindings rust crate.",
/// );
/// let project = endpoint.query(&client)?;
///
/// assert_eq!(project.name, "Traduora API rust bindings");
/// assert_eq!(project.description, "Translations for this Traduora API bindings rust crate.");
/// assert_eq!(project.terms_count, 23);
/// assert_eq!(project.locales_count, 3);
/// assert_eq!(project.role, Role::Admin);
/// assert_eq!(project.date.created, Utc.ymd(2021, 10, 24).and_hms_milli(15, 14, 30, 130));
/// assert_eq!(project.date.modified, Utc.ymd(2021, 10, 24).and_hms_milli(16, 51, 49, 000));
/// assert_eq!(project.id.value(), "1e7dfcea-85ff-4427-9401-aa2bbd99ac80");
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct EditProject {
    /// Unique id of the project to update.
    #[serde(skip_serializing)]
    pub id: ProjectId,
    /// Name of the project
    pub name: String,
    /// Short description for what the project is about.
    pub description: String,
}

impl EditProject {
    /// Create a new instance of the edit project endpoint.
    pub fn new(id: ProjectId, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}

impl Endpoint for EditProject {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for EditProject {
    type Model = Project;
}
