use http::Method;
use serde::{Deserialize, Serialize};

use crate::{auth::Authenticated, query::DefaultModel, Endpoint};

/// List all available locales.
///
/// **Endpoint** `GET /api/v1/locales`
///
/// **Default model** [`Locale`]
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use traduora::{api::locales::Locales, Query};
///
/// # let login = Login::password("tester@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let locales = Locales.query(&client)?;
///
/// assert!(locales.len() > 500);
/// let english = locales.iter().find(|l| l.code.value() == "en_US").unwrap();
/// assert_eq!(english.language, "English");
/// assert_eq!(english.region, "United States");
/// # Ok::<(), TraduoraError>(())
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Locales;

impl Endpoint for Locales {
    type AccessControl = Authenticated;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "locales".into()
    }
}

impl DefaultModel for Locales {
    type Model = Vec<Locale>;
}

impl_wrapper!(
    LocaleCode,
    "Type-safe wrapper for a standardized locale code (like `en_US`)."
);

/// A locale.
///
/// A locale is roughly equivalent to a language but it is
/// more specific. For instance `en_US` is english spoken in
/// the United States while `en_GB` is english spoken in
/// the United Kingdom.
///
/// Default model.
///
/// **Endpoint**
/// - `GET /api/v1/locales`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Locale {
    /// Unique code that identifies this locale.
    pub code: LocaleCode,
    /// Display string for the name of the language.
    pub language: String,
    /// Display string for the region where it is spoken.
    pub region: String,
}
