//! Permission levels.
//!
//! Types that implement the trait [Scope] represent a particular permission level.
//! Currently there are only two distinct levels:
//! * [Unauthenticated]
//! * [Authenticated]
//!
//! For a client, a higher access level allows it to access more endpoints.
//! For an endpoint, a higher access level prevents it from being accessed by
//! more clients.
//!
//! Having distinct types for the different scopes allows compile-time permission checks.

use http::{HeaderMap, HeaderValue};
use std::fmt::Debug;
use thiserror::Error;

use crate::api::auth::AccessToken;

/// The error which is returned from [`Scope::set_header`] when it failed to set the `Authorization` header.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// Header value could not be set.
    #[error("header value error: {}", source)]
    HeaderValue {
        /// Inner error.
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

/// Determines the permissions of a client.
/// What endpoints a client may access depends on its scope.
pub trait Scope {
    /// Adds the appropriate header to a set of headers.
    ///
    /// Depending on the token type, this will be either no header or the Authorization header.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    fn set_header<'a>(&self, headers: &'a mut HeaderMap) -> Result<&'a mut HeaderMap, AuthError>;
}

/// Client is authenticated and has an access token.
/// This allows calling all endpoints, including those that need authorization.
pub struct Authenticated(String);

/// Client is not authenticated. This means only a small subset of endpoints are available.
/// An endpoint with this scope can be queried without authentification.
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
    /// Downgrades an [Authenticated] scope to an [Unauthenticated] one.
    ///
    /// This method is mostly used for compile time type checking to
    /// ensure that an endpoint that requires authentification cannot be
    /// called before having acquired the authentification.
    fn from(_: Authenticated) -> Self {
        Unauthenticated
    }
}

impl From<AccessToken> for Authenticated {
    /// Constructs a new [Authenticated] scope from the given token.
    fn from(f: AccessToken) -> Self {
        Self(f.access_token)
    }
}

impl From<String> for Authenticated {
    /// Constructs a new [Authenticated] scope from the given token string.
    fn from(f: String) -> Self {
        Self(f)
    }
}
