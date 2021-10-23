use http::Method;
use serde::{Deserialize, Serialize};

use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Add a new project term
///
/// **Endpoint** `POST /api/v1/projects/{projectId}/terms`
///
/// **Default model** [`Term`]
///
/// # Examples
/// ```no_run
/// use traduora::{api::terms::CreateTerm, Query, Traduora, TraduoraError};
///
/// # fn main() -> Result<(), TraduoraError>{
/// let client = Traduora::new("localhost:8080")?;
/// let term = CreateTerm::new("this.is.a.new.term").query(&client)?;
/// assert_eq!(term.value, "this.is.a.new.term");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
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

/// Default model.
///
/// **Endpoint** `POST /api/v1/projects/{projectId}/terms`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    /// Unique id of the created term.
    pub id: api::TermId,
    /// The newly created term.
    pub value: String,
    /// Labels the term is tagged with.
    pub labels: Vec<String>,
    /// Timestamp about creation and last modification
    /// of this term.
    pub date: api::AccessDates,
}
