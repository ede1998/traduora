use serde::Deserialize;

use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Get the current user's profile.
///
/// **Endpoint** `GET /api/v1/users/me`
///
/// **Default model** [`UserInfo`]
///
/// # Examples
/// ```no_run
/// use traduora::{Query, TraduoraError, api::users::{Me, UserInfo}};
///
/// # fn main() -> Result<(), TraduoraError>{
/// # let client = traduora::DummyClient;
/// let user_info = Me.query(&client)?;
/// assert!(!user_info.id.is_empty());
/// # Ok(())
/// # }
/// ```
pub struct Me;

impl Endpoint for Me {
    type AccessControl = Authenticated;

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "users/me".into()
    }
}

impl DefaultModel for Me {
    type Model = UserInfo;
}

/// Default model.
///
/// **Endpoint** `GET /api/v1/users/me`
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// Unique id for the user.
    pub id: api::UserId,
    /// Name that is shown in the GUI.
    pub name: String,
    /// Email address associated with the account and used for login.
    pub email: String,
    /// Number of projects the user created.
    pub num_projects_created: u64,
}
