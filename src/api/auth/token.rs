//! See type [`Token`].

use http::Method;
use serde::{Deserialize, Serialize};

use crate::{api::Endpoint, auth::Unauthenticated, query::DefaultModel};

/// Request an authentication token for an existing user or project client.
///
/// **Endpoint** `/api/v1/auth/token`
///
/// # Examples
/// ```no_run
/// use traduora::{Query, TraduoraError, api::auth::{AccessToken, Token}};
///
/// # fn main() -> Result<(), TraduoraError>{
/// # let client = traduora::api::doctests::DummyClient;
/// let token = Token::password("user@traduora.example", "password").query(&client)?;
/// assert!(!token.access_token.is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum Token {
    /// Authentification for normal users. This uses the same login data that is typed in the browser.
    Password {
        /// Email address of the user.
        #[serde(rename = "username")]
        mail: String,
        /// Password of the user.
        password: String,
    },
    /// Authentification for project clients. This uses a special login for automated clients.
    /// See tab 'API Keys' within a project.
    ClientCredentials {
        /// Id of the project client.
        client_id: String,
        /// Secret of the project client.
        client_secret: String,
    },
}

impl Token {
    /// Constructs a new [`Token::Password`] variant.
    ///
    /// # Examples
    /// ```
    /// use traduora::api::auth::Token;
    ///
    /// let request = Token::password("user@traduora.example", "password");
    /// ```
    pub fn password<U, P>(mail: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Self::Password {
            mail: mail.into(),
            password: password.into(),
        }
    }

    /// Constructs a new [`Token::ClientCredentials`] variant.
    ///
    /// # Examples
    /// ```
    /// use traduora::api::auth::Token;
    ///
    /// let request = Token::client_credentials("f411de34-369d-436b-9aa6-4ae3d6d204be", "Hq4UFo6Z7sHODKdpAQEgaVR8onl8njLI");
    /// ```
    pub fn client_credentials<U, P>(id: U, secret: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Self::ClientCredentials {
            client_id: id.into(),
            client_secret: secret.into(),
        }
    }
}

impl std::fmt::Debug for Token {
    /// Formats the value using the given formatter. Sensitive data is expunged.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Password { mail, .. } => f
                .debug_struct("Password")
                .field("mail", mail)
                .field("password", &"***")
                .finish(),
            Self::ClientCredentials { client_id, .. } => f
                .debug_struct("ClientCredentials")
                .field("cliend_id", client_id)
                .field("client_secret", &"***")
                .finish(),
        }
    }
}

impl Endpoint for Token {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "auth/token".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::api::BodyError> {
        Ok(Some((
            "application/json",
            serde_json::to_string(self)?.into_bytes(),
        )))
    }

    type AccessControl = Unauthenticated;
}

impl DefaultModel for Token {
    type Model = AccessToken;

    fn map(data: serde_json::Value) -> Result<Self::Model, serde_json::Error> {
        serde_json::from_value(data)
    }
}

/// Default model.
///
/// **Endpoint** `/api/v1/auth/token`
#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccessToken {
    /// Authorization information for the client. To be sent in the `Authorization` header;
    pub access_token: String,
    /// Number of seconds after which the `access_token` expires.
    /// # Examples
    /// `86400s`
    pub expires_in: String,
    /// Type of access token. Should usually be `Bearer`.
    pub token_type: String,
}
