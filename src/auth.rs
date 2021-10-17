use std::fmt::Debug;

use http::{HeaderMap, HeaderValue};
use thiserror::Error;

use crate::api::auth::token::{AccessToken, AuthentificateRequest};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

/// Determines the permissions of a client.
pub trait Scope {
    /// Adds the appropriate header to a set of headers.
    ///
    /// Depending on the token type, this will be either the Private-Token header
    /// or the Authorization header.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    fn set_header<'a>(&self, headers: &'a mut HeaderMap) -> Result<&'a mut HeaderMap, AuthError>;
}

pub type Login = AuthentificateRequest;

/// Client is authenticated and has an access token.
/// This allows calling all endpoints, including those that need authorization.
pub struct Authenticated(String);

/// Client is not authenticated. This means only a small subset of endpoints are available.
pub struct Unauthenticated;

impl Scope for Authenticated {
    fn set_header<'a>(&self, headers: &'a mut HeaderMap) -> Result<&'a mut HeaderMap, AuthError> {
        let value = format!("Bearer {}", self.0);
        let mut token_header_value = HeaderValue::from_str(&value)?;
        token_header_value.set_sensitive(true);
        headers.insert(http::header::AUTHORIZATION, token_header_value);
        Ok(headers)
    }
}

impl Scope for Unauthenticated {
    fn set_header<'a>(&self, headers: &'a mut HeaderMap) -> Result<&'a mut HeaderMap, AuthError> {
        Ok(headers)
    }
}

impl From<Authenticated> for Unauthenticated {
    fn from(_: Authenticated) -> Self {
        Unauthenticated
    }
}

impl From<AccessToken> for Authenticated {
    fn from(f: AccessToken) -> Self {
        Self(f.access_token)
    }
}

impl From<String> for Authenticated {
    fn from(f: String) -> Self {
        Self(f)
    }
}
