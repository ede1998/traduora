use http::Method;

use super::Term;
use crate::{api::ProjectId, auth::Authenticated, query::DefaultModel, Endpoint};

/// List a project's terms.
///
/// **Endpoint** `GET /api/v1/projects/{projectId}/terms`
///
/// **Default model** [`Term`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use chrono::{TimeZone, Utc};
/// use traduora::{api::terms::Terms, api::labels::Label, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let terms = Terms("b1001dd9-e1c0-4fb0-a60d-eaaec304d332".into()).query(&client)?;
///
/// assert_eq!(terms.len(), 2);
/// assert_eq!(terms[0].value, "this.is.a.term");
/// assert_eq!(
///     terms[0].labels,
///     vec![Label {
///         id: "c16d0fc3-73e6-4962-b8d5-f3054b8ff002".into(),
///         value: "Example label".into(),
///         color: "#D81159".into()
///     }]
/// );
/// assert_eq!(terms[0].id.value(), "38ba819e-8023-464b-aa1b-6177c149f888");
/// assert_eq!(terms[0].date.created, Utc.ymd(2021, 10, 24).and_hms_milli(18, 43, 12, 131));
/// assert_eq!(terms[0].date.modified, Utc.ymd(2021, 10, 24).and_hms_milli(18, 43, 12, 131));
///
/// assert_eq!(terms[1].value, "this.is.another.term");
/// assert!(terms[1].labels.is_empty());
/// assert_eq!(terms[1].id.value(), "7eafe83d-1448-49ea-8ae0-f8753cbd669c");
/// assert_eq!(terms[1].date.created, Utc.ymd(2021, 10, 24).and_hms_milli(20, 02, 07, 952));
/// assert_eq!(terms[1].date.modified, Utc.ymd(2021, 10, 24).and_hms_milli(20, 02, 07, 952));
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Terms(pub ProjectId);

impl Endpoint for Terms {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("projects/{}/terms", self.0).into()
    }
}

impl DefaultModel for Terms {
    type Model = Vec<Term>;
}
