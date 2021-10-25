use serde::Deserialize;

use crate::api;

/// The translations of a specific term into a specific locale.
///
/// Default model.
///
/// **Endpoint**
/// - `GET /api/v1/projects/{projectId}/translations/{localeCode}`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    /// Unique id of the term.
    pub term_id: api::TermId,
    /// The translation of the term.
    pub value: String,
    /// Labels the term is tagged with.
    pub labels: Vec<String>,
    /// Timestamp about creation and last modification
    /// of this translation.
    pub date: api::AccessDates,
}
