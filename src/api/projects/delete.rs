use http::Method;
use serde::Serialize;

use crate::{api::ProjectId, auth::Authenticated, query::DefaultModel, Endpoint};

/// Delete a project.
///
/// **Endpoint** `DELETE /api/v1/projects`
///
/// **Default model** [`()`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::projects::DeleteProject, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let endpoint = DeleteProject("1e7dfcea-85ff-4427-9401-aa2bbd99ac80".into());
/// endpoint.query(&client)?;
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct DeleteProject(pub ProjectId);

impl Endpoint for DeleteProject {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}", self.0).into()
    }
}

impl DefaultModel for DeleteProject {
    type Model = ();

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}
