use http::Method;

use crate::{
    api::{locales::LocaleCode, ProjectId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Delete a project's locale.
///
/// **Endpoint** `DELETE /api/v1/projects/{projectId}/translations/{localeCode}`
///
/// **Default model** [`()`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::translations::DeleteLocale, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project_id = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let locale = "en_US".into();
/// DeleteLocale::new(project_id, locale).query(&client)?;
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct DeleteLocale {
    /// Unique id of the project the locale belongs to.
    pub project_id: ProjectId,
    /// Locale to delete.
    pub locale: LocaleCode,
}

impl DeleteLocale {
    /// Create a new instance of the delete locale endpoint.
    #[must_use]
    pub const fn new(project_id: ProjectId, locale: LocaleCode) -> Self {
        Self { project_id, locale }
    }
}

impl Endpoint for DeleteLocale {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/translations/{}", self.project_id, self.locale).into()
    }
}

impl DefaultModel for DeleteLocale {
    type Model = ();

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}
