use http::Method;

use super::Translation;
use crate::{
    api::{locales::LocaleCode, ProjectId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// List translated terms for a locale.
///
/// **Endpoint** `GET /api/v1/projects/{projectId}/translations/{localeCode}`
///
/// **Default model** [`Translation`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::translations::Translations, api::labels::Label, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let locale = "en_US".into();
/// let translations = Translations::new(project, locale).query(&client)?;
///
/// assert_eq!(translations.len(), 2);
/// assert_eq!(translations[0].value, "My first translation");
/// assert_eq!(
///     translations[0].labels,
///     vec![Label {
///         id: "c16d0fc3-73e6-4962-b8d5-f3054b8ff002".into(),
///         value: "Example label".into(),
///         color: "#D81159".into()
///     }]
/// );
/// assert_eq!(translations[0].term_id.value(), "38ba819e-8023-464b-aa1b-6177c149f888");
/// assert_eq!(translations[0].date.created, Utc.ymd(2021, 10, 25).and_hms_milli(18, 54, 16, 426));
/// assert_eq!(translations[0].date.modified, Utc.ymd(2021, 10, 25).and_hms_milli(18, 54, 16, 426));
///
/// assert_eq!(translations[1].value, "My second translation");
/// assert!(translations[1].labels.is_empty());
/// assert_eq!(translations[1].term_id.value(), "7eafe83d-1448-49ea-8ae0-f8753cbd669c");
/// assert_eq!(translations[1].date.created, Utc.ymd(2021, 10, 25).and_hms_milli(18, 54, 16, 422));
/// assert_eq!(translations[1].date.modified, Utc.ymd(2021, 10, 25).and_hms_milli(18, 54, 21, 000));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Translations {
    /// Unique id of the queried project.
    pub project_id: ProjectId,
    /// Locale code defining the language to list the translations for.
    pub locale_code: LocaleCode,
}

impl Translations {
    /// Creates a new instance of the translations endpoint.
    #[must_use]
    pub const fn new(project_id: ProjectId, locale_code: LocaleCode) -> Self {
        Self {
            project_id,
            locale_code,
        }
    }
}

impl Endpoint for Translations {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!(
            "projects/{}/translations/{}",
            self.project_id,
            self.locale_code.value()
        )
        .into()
    }
}

impl DefaultModel for Translations {
    type Model = Vec<Translation>;
}
