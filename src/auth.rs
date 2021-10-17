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

#[derive(Clone, Default)]
pub struct Auth(Option<AccessToken>);

impl From<AccessToken> for Auth {
    fn from(f: AccessToken) -> Self {
        Auth(Some(f))
    }
}

impl Auth {
    /// Adds the appropriate header to a set of headers.
    ///
    /// Depending on the token type, this will be either the Private-Token header
    /// or the Authorization header.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> Result<&'a mut HeaderMap<HeaderValue>, AuthError> {
        match &self.0 {
            Some(token) => {
                let value = format!("Bearer {}", token.access_token);
                let mut token_header_value = HeaderValue::from_str(&value)?;
                token_header_value.set_sensitive(true);
                headers.insert(http::header::AUTHORIZATION, token_header_value);
            }
            None => {}
        }
        Ok(headers)
    }
}

pub type Login = AuthentificateRequest;
