use http::Method;
use serde::Serialize;

use super::Project;
use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Creates a new project and assigns the requesting user as the admin.
///
/// **Endpoint** `POST /api/v1/projects`
///
/// **Default model** [`Project`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::{projects::CreateProject, Role}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let endpoint = CreateProject::new("Traduora API bindings", "Translations for this Traduora API bindings rust crate.");
/// let project = endpoint.query(&client)?;
///
/// assert_eq!(project.name, "Traduora API bindings");
/// assert_eq!(project.description, "Translations for this Traduora API bindings rust crate.");
/// assert_eq!(project.terms_count, 0);
/// assert_eq!(project.locales_count, 0);
/// assert_eq!(project.role, Role::Admin);
/// assert_eq!(project.date.created, Utc.ymd(2021, 10, 24).and_hms_milli(15, 14, 30, 130));
/// assert_eq!(project.date.modified, Utc.ymd(2021, 10, 24).and_hms_milli(15, 14, 30, 130));
/// assert_eq!(project.id.value(), "1e7dfcea-85ff-4427-9401-aa2bbd99ac80");
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct CreateProject {
    /// Name of the project
    pub name: String,
    /// Short description for what the project is about.
    pub description: String,
}

impl CreateProject {
    /// Create a new instance of the project creation endpoint.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}

impl Endpoint for CreateProject {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "projects".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for CreateProject {
    type Model = Project;
}
