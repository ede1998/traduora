use http::Method;
use serde::{Deserialize, Serialize};

use crate::{auth::Unauthenticated, query::DefaultModel, Endpoint};

/// List available external auth providers.
///
/// **Endpoint** `GET /api/v1/auth/providers`
///
/// # Examples
/// ```no_run
/// use traduora::{api::auth::Providers, Query, Traduora, TraduoraError};
///
/// # fn main() -> Result<(), TraduoraError>{
/// let client = Traduora::new("localhost:8080")?;
/// let providers = Providers.query(&client)?;
/// println!("Providers: {:?}", providers);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Providers;

impl Endpoint for Providers {
    type AccessControl = Unauthenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "auth/providers".into()
    }
}

impl DefaultModel for Providers {
    type Model = Vec<AuthProvider>;
}

/// Default model.
///
/// **Endpoint** `GET /api/v1/auth/providers`
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthProvider {
    /// DONT KNOW
    pub slug: String,
    /// DONT KNOW
    pub client_id: String,
    /// DONT KNOW
    pub url: String,
    /// DONT KNOW
    pub redirect_url: String,
}
