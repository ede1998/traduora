use http::Method;
use serde::Serialize;

use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Change password of the logged in user using current password.
///
/// **Endpoint** `POST /api/v1/auth/change-password`
///
/// **Default model** `()`
///
/// # Examples
/// ```
/// # use traduora::{TestClient as Traduora, TraduoraError};
/// use traduora::{api::auth::ChangePassword, Login, Query};
///
/// let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let password_change = ChangePassword::new("letmeinpls", "muchmoresecure");
/// password_change.query(&client)?;
///
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    /// The current password of the user.
    pub old_password: String,
    /// The new password for the user.
    pub new_password: String,
}

impl ChangePassword {
    /// Creates a new instance of the change password endpoint.
    pub fn new(old: impl Into<String>, new: impl Into<String>) -> Self {
        Self {
            old_password: old.into(),
            new_password: new.into(),
        }
    }
}

impl Endpoint for ChangePassword {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "auth/change-password".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for ChangePassword {
    type Model = ();

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}
