use http::Method;
use serde::{Deserialize, Serialize};

use crate::{auth::Unauthenticated, query::DefaultModel, Endpoint};

/// Create a new user account.
///
/// **Endpoint** `POST /api/v1/auth/signup`
///
/// # Examples
/// ```no_run
/// use traduora::{api::auth::Signup, Query, Traduora, TraduoraError};
///
/// # fn main() -> Result<(), TraduoraError>{
/// let client = Traduora::new("localhost:8080")?;
/// let new_user = Signup::new("tester", "test@traduora.example", "letmeinpls").query(&client)?;
/// assert_eq!(new_user.email, "test@traduora.example");
/// assert_eq!(new_user.name, "tester");
/// # Ok(())
/// # }
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
    pub id: String,
    /// Name of the newly created user.
    pub name: String,
    /// Email address of the newly created user.
    pub email: String,
    /// Token to use endpoints that require authentification.
    pub access_token: String,
}
