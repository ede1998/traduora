use http::Method;
use serde::{Deserialize, Serialize};

use crate::{api, auth::Unauthenticated, query::DefaultModel, Endpoint};

/// Create a new user account.
///
/// **Endpoint** `POST /api/v1/auth/signup`
///
/// **Default model** [`NewUser`]
///
/// # Examples
/// ```
/// # use traduora::{TestClient as Traduora, TraduoraError};
/// use traduora::{api::auth::Signup, Query};
///
/// let client = Traduora::new("localhost:8080")?;
/// let signup = Signup::new("Tester", "tester@mail.example", "letmeinpls");
/// let new_user = signup.query(&client)?;
///
/// assert_eq!(new_user.name, "Tester");
/// assert_eq!(new_user.email, "tester@mail.example");
/// assert_eq!(new_user.id.value(), "2916cc62-5080-4b9f-b0ab-82d72e938004");
/// assert!(new_user.access_token.value().starts_with("eyJhbGciOiJIUzI1NiIs"));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Signup {
    /// Display name for the user to create.
    pub name: String,
    /// Login email address for the user to create.
    pub email: String,
    /// Login password for the user to create.
    pub password: String,
}

impl Signup {
    /// Creates a new instance of the signup endpoint.
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            password: password.into(),
            email: email.into(),
        }
    }
}

impl Endpoint for Signup {
    type AccessControl = Unauthenticated;

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "auth/signup".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::BodyError> {
        Ok(Some((
            api::mime_types::JSON,
            serde_json::to_string(self)?.into_bytes(),
        )))
    }
}

impl DefaultModel for Signup {
    type Model = NewUser;
}

/// Default model.
///
/// **Endpoint** `GET /api/v1/auth/providers`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    /// Unique id of the created user.
    pub id: api::UserId,
    /// Name of the newly created user.
    pub name: String,
    /// Email address of the newly created user.
    pub email: String,
    /// Token to use endpoints that require authentification.
    pub access_token: api::AccessToken,
}
