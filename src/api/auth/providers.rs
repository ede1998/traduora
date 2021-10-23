use http::Method;
use serde::{Deserialize, Serialize};

use crate::{auth::Unauthenticated, query::DefaultModel, Endpoint};

/// List available external auth providers.
///
/// The exact result depends on what external authentication
/// providers are configured in the Traduora instance.
/// By default there are none.
///
/// **Endpoint** `GET /api/v1/auth/providers`
///
/// **Default model** [`AuthProvider`]
///
/// # Examples
/// ```
/// # use traduora::{TestClient as Traduora, TraduoraError};
/// use traduora::{api::auth::Providers, Query};
///
/// let client = Traduora::new("localhost:8080")?;
/// let providers = Providers.query(&client)?;
///
/// assert_eq!(providers[0].slug, "google");
/// assert_eq!(providers[0].client_id, "1234567890-abc123def456.apps.googleusercontent.com");
/// assert_eq!(providers[0].url.as_str(), "https://accounts.google.com/o/oauth2/v2/auth");
/// assert_eq!(providers[0].redirect_url.as_str(), "https://www.traduora.example/auth/callback");
/// # Ok::<(), TraduoraError>(())
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

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}

/// Default model.
///
/// **Endpoint** `GET /api/v1/auth/providers`
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthProvider {
    /// Name of the authentication provider
    pub slug: String,
    /// The id of the traduora instance in the authentication
    /// provider.
    pub client_id: String,
    /// URL where the authentication with this external provider happens.
    pub url: url::Url,
    /// URL that is called after the authentication finishes to
    /// return back to the Traduora instance.
    pub redirect_url: url::Url,
}
