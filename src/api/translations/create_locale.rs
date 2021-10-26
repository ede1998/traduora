use http::Method;
use serde::Serialize;

use super::ProjectLocale;
use crate::{
    api::{self, locales::LocaleCode, ProjectId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Add a new translation locale for a project.
///
/// **Endpoint** `POST /api/v1/projects/{projectId}/translations`
///
/// **Default model** [`ProjectLocale`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::{ProjectId, translations::CreateLocale}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project_id = ProjectId::new("b1001dd9-e1c0-4fb0-a60d-eaaec304d332");
/// let locale = CreateLocale::new(project_id, "en_US".into()).query(&client)?;
///
/// assert_eq!(locale.locale.code.value(), "en_US");
/// assert_eq!(locale.id.value(), "5bda98b2-849c-49ff-b162-70fd707f6dce");
/// // assert_eq!(term.date.created, Utc::now());
/// // assert_eq!(term.date.modified, Utc::now());
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct CreateLocale {
    /// Project for which the locale should be created.
    #[serde(skip_serializing)]
    pub project: ProjectId,
    /// Code of the locale to create.
    pub code: LocaleCode,
}

impl CreateLocale {
    /// Creates a new instance of the [`CreateLocale`] endpoint.
    #[must_use]
    pub const fn new(project: ProjectId, code: LocaleCode) -> Self {
        Self { project, code }
    }
}

impl Endpoint for CreateLocale {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/translations", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for CreateLocale {
    type Model = ProjectLocale;
}
