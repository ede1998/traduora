use serde::Deserialize;

use crate::api;

/// Default model.
///
/// **Endpoint**
/// - `POST /api/v1/projects/{projectId}/terms`
/// - `GET /api/v1/projects/{projectId}/terms`
/// - `PATCH /api/v1/projects/{projectId}/terms/{termId}`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    /// Unique id of the created term.
    pub id: api::TermId,
    /// The newly created term.
    pub value: String,
    /// Labels the term is tagged with.
    pub labels: Vec<api::labels::Label>,
    /// Timestamp about creation and last modification
    /// of this term.
    pub date: api::AccessDates,
}
