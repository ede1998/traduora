use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

impl_wrapper!(AccessToken, "Type-safe access token wrapper");
impl_wrapper!(UserId, "Type-safe user id wrapper");
impl_wrapper!(ProjectId, "Type-safe project id wrapper");
impl_wrapper!(TermId, "Type-safe term id wrapper");

/// Data object that is returned by the Traduora API
/// for multiple endpoints.
///
/// It contains timestamps that inform about the latest
/// interactions with an object. What exactly an object
/// is depends on the endpoint that return the [`AccessDates`].
#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessDates {
    /// Time when the object was created.
    pub created: DateTime<Utc>,
    /// Time when the object was last changed.
    pub modified: DateTime<Utc>,
}

/// Data object that holds a role. These roles are project-specific for each user.
/// By default, the creator of a new project becomes its admin.
/// All other users get the role that the project admin chose for them while inviting.
///
/// For a detailed overview of what role may access which endpoint, see
/// <https://docs.traduora.co/docs/api/v1/roles-permissions>.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    /// Access to everything
    Admin,
    /// Read and write access but may not change
    /// configuration data like inviting a new user.
    Editor,
    /// Read-only access
    Viewer,
}

pub(crate) mod mime_types {
    pub const JSON: &str = "application/json";
}
