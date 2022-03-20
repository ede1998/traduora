use http::Method;
use serde::Deserialize;

use crate::{
    api::{locales::Locale, AccessDates, ProjectId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// List all translations locales for a project.
///
/// **Endpoint** `GET /api/v1/projects/{projectId}/translations`
///
/// **Default model** [`ProjectLocale`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::translations::Locales, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let locales = Locales(project).query(&client)?;
///
/// assert_eq!(locales.len(), 2);
/// assert_eq!(locales[0].locale.code, "de_DE".into());
/// assert_eq!(locales[0].id.value(), "91ea2d61-8e14-481f-af19-9ae16bbf95a3");
/// assert_eq!(locales[0].date.created, Utc.ymd(2021, 10, 26).and_hms_milli(20, 15, 54, 194));
/// assert_eq!(locales[0].date.modified, Utc.ymd(2021, 10, 26).and_hms_milli(20, 15, 54, 194));
///
/// assert_eq!(locales[1].locale.code, "en".into());
/// assert_eq!(locales[1].id.value(), "38c02384-9be0-4308-aac6-056f1d7b0a08");
/// assert_eq!(locales[1].date.created, Utc.ymd(2021, 10, 26).and_hms_milli(20, 16, 03, 781));
/// assert_eq!(locales[1].date.modified, Utc.ymd(2021, 10, 26).and_hms_milli(20, 16, 03, 781));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Locales(pub ProjectId);

impl Endpoint for Locales {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/translations", self.0).into()
    }
}

impl DefaultModel for Locales {
    type Model = Vec<ProjectLocale>;
}

impl_wrapper!(
    ProjectLocaleId,
    "Type-safe wrapper for a project locale id."
);

/// A project's locale. Contains a generic [`Locale`] as well
/// as an id and creation/modification times.
///
/// Default model.
///
/// **Endpoint**
/// - `GET /api/v1/projects/{projectId}/translations`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLocale {
    /// Unique id.
    pub id: ProjectLocaleId,
    /// Generic locale.
    pub locale: Locale,
    /// Modification and creation times.
    pub date: AccessDates,
}
