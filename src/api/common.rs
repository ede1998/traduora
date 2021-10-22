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

pub(crate) mod mime_types {
    pub const JSON: &str = "application/json";
}
