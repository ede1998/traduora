use serde::Deserialize;

use crate::api::{AccessDates, ProjectId, Role};

/// A Traduora project.
/// Each project contains a collection of terms and their translations
/// into various locales.
///
/// Default model.
///
/// **Endpoint**
/// - `GET /api/v1/projects`
/// - `GET /api/v1/projects/{projectId}`
/// - `POST /api/v1/projects`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Unique id of the project.
    pub id: ProjectId,
    /// Display name for the project.
    pub name: String,
    /// More detailed description of what the project is about.
    pub description: String,
    /// Number of locales (=languages) that are configured for this project.
    pub locales_count: u64,
    /// Number of terms the project owns.
    pub terms_count: u64,
    /// Role of the querying user within the project.
    pub role: Role,
    /// Time when the project was created and last modified.
    pub date: AccessDates,
}
