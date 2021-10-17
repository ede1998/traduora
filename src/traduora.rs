// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::any;
use std::convert::TryInto;
use std::fmt::{self, Debug};

use async_trait::async_trait;
use bytes::Bytes;
use http::Response as HttpResponse;
use log::{debug, error};
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;
use thiserror::Error;
use url::Url;

use crate::api::{self, auth::token::AccessToken, AsyncQuery, Query};
use crate::auth::{Auth, AuthError, Login};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TraduoraError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("communication with gitlab: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("gitlab HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },
    #[error("no response from gitlab")]
    NoResponse {},
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },
    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
}

impl TraduoraError {
    fn http(status: reqwest::StatusCode) -> Self {
        TraduoraError::Http { status }
    }

    fn no_response() -> Self {
        TraduoraError::NoResponse {}
    }

    fn data_type<T>(source: serde_json::Error) -> Self {
        TraduoraError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

type TraduoraResult<T> = Result<T, TraduoraError>;

/// A representation of the Traduora API for a single user.
///
/// Separate users should use separate instances of this.
#[derive(Clone)]
pub struct Traduora {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Traduora.
    token: Auth,
}

impl Debug for Traduora {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Traduora")
            .field("rest_url", &self.rest_url)
            .finish()
    }
}

/// Should a certificate be validated in tls connections.
/// The Insecure option is used for self-signed certificates.
#[derive(Debug, Clone)]
enum CertPolicy {
    Default,
    Insecure,
}

impl Traduora {
    /// Create a new Traduora API representation.
    ///
    /// Errors out if `auth` is invalid.
    pub fn new<T>(host: T, auth: impl Into<Option<Login>>) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Self::new_impl("https", host.as_ref(), auth.into(), CertPolicy::Default)
    }

    /// Create a new non-SSL Traduora API representation.
    ///
    /// Errors out if `auth` is invalid.
    pub fn new_insecure<T>(host: T, auth: impl Into<Option<Login>>) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Self::new_impl("http", host.as_ref(), auth.into(), CertPolicy::Insecure)
    }

    /// Internal method to create a new Traduora client.
    fn new_impl(
        protocol: &str,
        host: &str,
        auth: Option<Login>,
        cert_validation: CertPolicy,
    ) -> TraduoraResult<Self> {
        let rest_url = Url::parse(&format!("{}://{}/api/v1/", protocol, host))?;

        let client = match cert_validation {
            CertPolicy::Insecure => Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?,
            CertPolicy::Default => Client::new(),
        };

        let mut api = Traduora {
            client,
            rest_url,
            token: Default::default(),
        };

        if let Some(auth) = auth {
            let token: AccessToken = auth.query(&api)?;
            api.token = token.into();
        }

        Ok(api)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("communication with gitlab: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

impl api::RestClient for Traduora {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "gitlab", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl api::Client for Traduora {
    fn rest(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            self.token.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(api::ApiError::client)
    }
}

/// A representation of the asynchronous Traduora API for a single user.
///
/// Separate users should use separate instances of this.
#[derive(Clone)]
pub struct AsyncTraduora {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Traduora.
    token: Auth,
}

impl Debug for AsyncTraduora {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AsyncTraduora")
            .field("rest_url", &self.rest_url)
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncTraduora {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "gitlab", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }
}

#[async_trait]
impl api::AsyncClient for AsyncTraduora {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            self.token.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().await.map_err(api::ApiError::client)
    }
}

impl AsyncTraduora {
    /// Create a new Traduora API representation.
    ///
    /// Errors out if `auth` is invalid.
    pub async fn new<T>(host: T, auth: impl Into<Option<Login>>) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Self::new_impl("https", host.as_ref(), auth.into(), CertPolicy::Default).await
    }

    /// Create a new non-SSL Traduora API representation.
    ///
    /// Errors out if `auth` is invalid.
    pub async fn new_insecure<T>(host: T, auth: impl Into<Option<Login>>) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Self::new_impl("http", host.as_ref(), auth.into(), CertPolicy::Insecure).await
    }

    /// Internal method to create a new Traduora client.
    async fn new_impl(
        protocol: &str,
        host: &str,
        auth: Option<Login>,
        cert_validation: CertPolicy,
    ) -> TraduoraResult<Self> {
        let rest_url = Url::parse(&format!("{}://{}/api/v1/", protocol, host))?;

        let client = match cert_validation {
            CertPolicy::Insecure => AsyncClient::builder()
                .danger_accept_invalid_certs(true)
                .build()?,
            CertPolicy::Default => AsyncClient::new(),
        };

        let mut api = AsyncTraduora {
            client,
            rest_url,
            token: Default::default(),
        };

        if let Some(auth) = auth {
            let token: AccessToken = auth.query_async(&api).await?;
            api.token = token.into();
        }

        Ok(api)
    }
}
