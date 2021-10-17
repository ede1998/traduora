use http::Method;
use serde::{Deserialize, Serialize};

use crate::{api::Endpoint, auth::Unauthenticated};

#[derive(Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: String,
    pub token_type: String,
}

/// Traduora API token
///
/// Tradura supports two kinds of authentifications.
#[derive(Clone, Serialize)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum AuthentificateRequest {
    /// Authentification for users.
    Password {
        #[serde(rename = "username")]
        mail: String,
        password: String,
    },
    /// Authentification for project clients.
    ClientCredentials {
        client_id: String,
        client_secret: String,
    },
}

impl AuthentificateRequest {
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

impl std::fmt::Debug for AuthentificateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Password { mail: user, .. } => {
                f.debug_struct("Password").field("user", user).finish()
            }
            Self::ClientCredentials { client_id, .. } => f
                .debug_struct("ClientCredentials")
                .field("cliend_id", client_id)
                .finish(),
        }
    }
}

impl Endpoint for AuthentificateRequest {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn has_secrets(&self) -> bool {
        false
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
