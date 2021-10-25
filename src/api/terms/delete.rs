use http::Method;

use crate::{
    api::{ProjectId, TermId},
    auth::Authenticated,
    query::DefaultModel,
    Endpoint,
};

/// Remove a project's term.
///
/// **Endpoint** `DELETE /api/v1/projects/{projectId}/terms/{termId}`
///
/// **Default model** [`()`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::terms::DeleteTerm, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let project_id = "b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into();
/// let term_id = "0fa39756-65db-423c-a6d9-534b62fe9ead".into();
/// DeleteTerm::new(project_id, term_id).query(&client)?;
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct DeleteTerm {
    /// Unique id of the project the term belongs to.
    pub project_id: ProjectId,
    /// Unique id of the term to delete.
    pub term_id: TermId,
}

impl DeleteTerm {
    /// Create a new instance of the delete term endpoint.
    #[must_use]
    pub const fn new(project_id: ProjectId, term_id: TermId) -> Self {
        Self {
            project_id,
            term_id,
        }
    }
}

impl Endpoint for DeleteTerm {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/terms/{}", self.project_id, self.term_id).into()
    }
}

impl DefaultModel for DeleteTerm {
    type Model = ();

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}
