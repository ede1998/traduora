use http::Method;
use serde::Serialize;

use super::UserInfo;
use crate::{api, auth::Authenticated, query::DefaultModel, Endpoint};

/// Update the current user's profile.
///
/// Note that the returned [`UserInfo`] seems to be missing the
/// `num_projects_created` field.
///
/// **Endpoint** `PATCH /api/v1/users/me`
///
/// **Default model** [`UserInfo`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::users::EditMe, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let endpoint = EditMe::name("Tester 2");
/// let user = endpoint.query(&client)?;
///
/// assert_eq!(user.id.value(), "40379230-ced0-43b8-8b78-37c924f491a7");
/// assert_eq!(user.name, "Tester 2");
/// assert_eq!(user.email, "test@mail.example");
/// assert_eq!(user.num_projects_created, None);
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize, Default)]
pub struct EditMe {
    /// New name for the user.
    pub name: Option<String>,
    /// New mail for the user.
    pub email: Option<String>,
}

impl EditMe {
    /// Create a new instance of the edit me endpoint to edit both
    /// name and email.
    pub fn name_and_mail(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            email: Some(email.into()),
        }
    }

    /// Create a new instance of the edit me endpoint to only update the mail address.
    pub fn email(email: impl Into<String>) -> Self {
        Self {
            email: Some(email.into()),
            name: None,
        }
    }

    /// Create a new instance of the edit me endpoint to only update the name address.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            email: None,
        }
    }
}

impl Endpoint for EditMe {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "users/me".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for EditMe {
    type Model = UserInfo;
}
