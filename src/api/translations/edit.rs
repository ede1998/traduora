use http::Method;
use serde::Serialize;

use super::Translation;
use crate::{
    api::{self, locales::LocaleCode, ProjectId, TermId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Update a term's translation.
///
/// **Endpoint** `PATCH /api/v1/projects/{projectId}/translations/{localeCode}`
///
/// **Default model** [`Translation`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::{translations::EditTranslation, Role}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let term = "7eafe83d-1448-49ea-8ae0-f8753cbd669c".into();
/// let locale = "en_US".into();
/// let endpoint = EditTranslation::new(project, locale, term, "New translation");
/// let translation = endpoint.query(&client)?;
///
/// assert_eq!(translation.value, "New translation");
/// assert_eq!(translation.labels, vec!["somelabel"]);
/// assert_eq!(translation.term_id.value(), "7eafe83d-1448-49ea-8ae0-f8753cbd669c");
/// assert_eq!(translation.date.created, Utc.ymd(2021, 10, 25).and_hms_milli(18, 54, 16, 422));
/// assert_eq!(translation.date.modified, Utc.ymd(2021, 10, 26).and_hms_milli(16, 03, 09, 000));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTranslation {
    /// Unique id of the project the term belongs to.
    #[serde(skip_serializing)]
    pub project_id: ProjectId,
    /// Locale the translation belongs to.
    #[serde(skip_serializing)]
    pub locale: LocaleCode,
    /// Unique id of the term to update.
    pub term_id: TermId,
    /// New translation string.
    pub value: String,
}

impl EditTranslation {
    /// Create a new instance of the edit translation endpoint.
    pub fn new(
        project_id: ProjectId,
        locale: LocaleCode,
        term_id: TermId,
        value: impl Into<String>,
    ) -> Self {
        Self {
            project_id,
            locale,
            term_id,
            value: value.into(),
        }
    }
}

impl Endpoint for EditTranslation {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/translations/{}", self.project_id, self.locale).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for EditTranslation {
    type Model = Translation;
}
