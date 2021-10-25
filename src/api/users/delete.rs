use http::Method;

use crate::{auth::Authenticated, query::DefaultModel, Endpoint};

/// Delete the current user's account.
///
/// **Endpoint** `DELETE /api/v1/me`
///
/// **Default model** [`()`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::users::DeleteMe, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// DeleteMe.query(&client)?;
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct DeleteMe;

impl Endpoint for DeleteMe {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "users/me".into()
    }
}

impl DefaultModel for DeleteMe {
    type Model = ();

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}
