use serde::Deserialize;

use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Get the current user's profile.
///
/// **Endpoint** `GET /api/v1/users/me`
///
/// **Default model** [`UserInfo`]
///
/// # Examples
/// ```
/// # use traduora::{TestClient as Traduora, TraduoraError};
/// use traduora::{api::users::Me, Login, Query};
///
/// let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let user_info = Me.query(&client)?;
///
/// assert_eq!(user_info.name, "Tester");
/// assert_eq!(user_info.email, "tester@mail.example");
/// assert_eq!(user_info.num_projects_created, Some(1));
/// assert_eq!(user_info.id.value(), "40379230-ced0-43b8-8b78-37c924f491a7");
/// # Ok::<(), TraduoraError>(())
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
/// **Endpoint**
/// - `GET /api/v1/users/me`
/// - `PATCH /api/v1/user/me`
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// Unique id for the user.
    pub id: api::UserId,
    /// Name that is shown in the GUI.
    pub name: String,
    /// Email address associated with the account and used for login.
    pub email: String,
    /// Number of projects the user created. If it's none,
    /// the API didn't provide this value.
    pub num_projects_created: Option<u64>,
}
