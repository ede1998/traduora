use http::Method;
use serde::Serialize;

use super::Term;
use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Add a new project term
///
/// **Endpoint** `POST /api/v1/projects/{projectId}/terms`
///
/// **Default model** [`Term`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::{ProjectId, terms::CreateTerm}, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let some_project_id = ProjectId::new("b1001dd9-e1c0-4fb0-a60d-eaaec304d332");
/// let term = CreateTerm::new("this.is.a.new.term", some_project_id).query(&client)?;
///
/// assert_eq!(term.value, "this.is.a.new.term");
/// assert!(term.labels.is_empty());
/// assert_eq!(term.id.value(), "b686f455-b668-40f7-860d-8828263fc8c0");
/// // assert_eq!(term.date.created, Utc::now());
/// // assert_eq!(term.date.modified, Utc::now());
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct CreateTerm {
    /// The string that should become a term.
    pub term: String,
    /// Project for which the term should be created.
    pub project: api::ProjectId,
}

impl CreateTerm {
    /// Creates a new instance of the [`CreateTerm`] endpoint.
    pub fn new(term: impl Into<String>, project: impl Into<api::ProjectId>) -> Self {
        Self {
            term: term.into(),
            project: project.into(),
        }
    }
}

impl Endpoint for CreateTerm {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/terms", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        #[derive(Serialize)]
        struct Dto<'a> {
            pub value: &'a str,
        }

        let dto = Dto { value: &self.term };

        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(&dto)?.into_bytes(),
        )))
    }
}

impl DefaultModel for CreateTerm {
    type Model = Term;
}
