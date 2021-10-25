use http::Method;
use serde::Serialize;

use super::Term;
use crate::{
    api::{self, ProjectId, TermId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Update a project's term.
///
/// **Endpoint** `PATCH /api/v1/projects/{projectId}/terms/{termId}`
///
/// **Default model** [`Term`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::terms::EditTerm, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project_id = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let term_id = "0fa39756-65db-423c-a6d9-534b62fe9ead".into();
/// let term = EditTerm::new(project_id, term_id, "new.term.text").query(&client)?;
///
/// assert_eq!(term.value,  "new.term.text");
/// assert_eq!(term.labels, ["somelabel"]);
/// assert_eq!(term.id.value(), "0fa39756-65db-423c-a6d9-534b62fe9ead");
/// assert_eq!(term.date.created, Utc.ymd(2021, 10, 23).and_hms_milli(16, 12, 55, 691));
/// assert_eq!(term.date.modified, Utc.ymd(2021, 10, 25).and_hms_milli(16, 10, 16, 000));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct EditTerm {
    /// Unique id of the project the term belongs to.
    #[serde(skip_serializing)]
    pub project_id: ProjectId,
    /// Unique id of the term to update.
    #[serde(skip_serializing)]
    pub term_id: TermId,
    /// New value for the term string.
    pub value: String,
}

impl EditTerm {
    /// Create a new instance of the edit term endpoint.
    pub fn new(project_id: ProjectId, term_id: TermId, value: impl Into<String>) -> Self {
        Self {
            project_id,
            term_id,
            value: value.into(),
        }
    }
}

impl Endpoint for EditTerm {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/terms/{}", self.project_id, self.term_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for EditTerm {
    type Model = Term;
}
