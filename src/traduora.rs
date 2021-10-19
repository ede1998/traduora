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

use crate::api;
use crate::auth::{AuthError, Authenticated, Scope, Unauthenticated};
use crate::query::{AsyncQuery, Query};
use crate::Login;

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
    #[error("communication with traduora: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("traduora HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },
    #[error("no response from traduora")]
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

type TraduoraResult<T> = Result<T, TraduoraError>;

/// A representation of the Traduora API for a single user.
///
/// Separate users should use separate instances of this.
#[derive(Clone)]
pub struct Traduora<A: Scope> {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Traduora.
    token: A,
}

impl<A: Scope + Debug> Debug for Traduora<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Traduora")
            .field("rest_url", &self.rest_url)
            .field("token", &format!("{:?}", self.token))
            .finish()
    }
}

impl Traduora<Unauthenticated> {
    /// Create a new Traduora API representation.
    pub fn new<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).build()
    }

    /// Create a new non-SSL Traduora API representation.
    pub fn new_insecure<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .build()
    }

    pub fn authenticate(self, login: &Login) -> TraduoraResult<Traduora<Authenticated>> {
        let token = login.query(&self)?;

        Ok(Traduora {
            client: self.client,
            rest_url: self.rest_url,
            token: token.into(),
        })
    }
}

impl Traduora<Authenticated> {
    /// Create a new Traduora API representation.
    pub fn with_auth<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).authenticate(login).build()
    }

    /// Create a new non-SSL Traduora API representation.
    pub fn with_auth_insecure<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .authenticate(login)
            .build()
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
    #[error("communication with traduora: {}", source)]
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

impl<A: Scope> api::RestClient for Traduora<A> {
    type Error = RestError;
    type AccessLevel = A;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "traduora", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl<A: Scope> api::Client for Traduora<A> {
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
pub struct AsyncTraduora<A: Scope> {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Traduora.
    token: A,
}

impl<A: Scope + Debug> Debug for AsyncTraduora<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AsyncTraduora")
            .field("rest_url", &self.rest_url)
            .field("token", &format!("{:?}", self.token))
            .finish()
    }
}

#[async_trait]
impl<A: Scope> api::RestClient for AsyncTraduora<A> {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "traduora", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }

    type AccessLevel = Authenticated;
}

#[async_trait]
impl<A: Scope + Send + Sync> api::AsyncClient for AsyncTraduora<A> {
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

impl AsyncTraduora<Unauthenticated> {
    /// Create a new Traduora API representation.
    pub fn new<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).build_async()
    }

    /// Create a new non-SSL Traduora API representation.
    pub fn new_insecure<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .build_async()
    }

    pub async fn authenticate(self, login: &Login) -> TraduoraResult<AsyncTraduora<Authenticated>> {
        let token = login.query_async(&self).await?;

        Ok(AsyncTraduora {
            client: self.client,
            rest_url: self.rest_url,
            token: token.into(),
        })
    }
}

impl AsyncTraduora<Authenticated> {
    /// Create a new Traduora API representation.
    pub async fn with_auth<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str> + Sync + Send + 'static,
    {
        Builder::new(host.as_ref())
            .authenticate(login)
            .build_async()
            .await
    }

    /// Create a new non-SSL Traduora API representation.
    pub async fn with_auth_insecure<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str> + Sync + Send + 'static,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .authenticate(login)
            .build_async()
            .await
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct Builder<'h, L> {
    host: &'h str,
    protocol: &'static str,
    validate_certs: bool,
    login: L,
}

impl<'h> Builder<'h, ()> {
    pub const fn new(host: &'h str) -> Self {
        Self {
            host,
            protocol: "https",
            validate_certs: true,
            login: (),
        }
    }

    pub const fn authenticate(self, login: Login) -> Builder<'h, Login> {
        Builder {
            host: self.host,
            protocol: self.protocol,
            validate_certs: self.validate_certs,
            login,
        }
    }

    pub fn with_access_token(self, token: impl Into<String>) -> Builder<'h, String> {
        Builder {
            host: self.host,
            protocol: self.protocol,
            validate_certs: self.validate_certs,
            login: token.into(),
        }
    }

    pub fn build(&self) -> TraduoraResult<Traduora<Unauthenticated>> {
        self.build_unauthenticated()
    }

    pub fn build_async(&self) -> TraduoraResult<AsyncTraduora<Unauthenticated>> {
        self.build_unauthenticated_async()
    }
}

impl<'h> Builder<'h, Login> {
    pub fn build(self) -> TraduoraResult<Traduora<Authenticated>> {
        let api = self.build_unauthenticated()?;
        api.authenticate(&self.login)
    }

    pub async fn build_async(self) -> TraduoraResult<AsyncTraduora<Authenticated>> {
        let api = self.build_unauthenticated_async()?;
        api.authenticate(&self.login).await
    }
}

impl<'h> Builder<'h, String> {
    pub fn build(&self) -> TraduoraResult<Traduora<Authenticated>> {
        let api = self.build_unauthenticated()?;
        Ok(Traduora {
            client: api.client,
            rest_url: api.rest_url,
            token: self.login.clone().into(),
        })
    }

    pub async fn build_async(&self) -> TraduoraResult<AsyncTraduora<Authenticated>> {
        let api = self.build_unauthenticated_async()?;
        Ok(AsyncTraduora {
            client: api.client,
            rest_url: api.rest_url,
            token: self.login.clone().into(),
        })
    }
}

impl<'h, L> Builder<'h, L> {
    pub const fn use_http(mut self, use_http: bool) -> Self {
        self.protocol = if use_http { "http" } else { "https" };
        self
    }

    pub const fn validate_certs(mut self, validate: bool) -> Self {
        self.validate_certs = validate;
        self
    }

    fn build_rest_url(&self) -> Result<Url, url::ParseError> {
        format!("{}://{}/api/v1/", self.protocol, self.host).parse()
    }

    fn build_unauthenticated(&self) -> TraduoraResult<Traduora<Unauthenticated>> {
        Ok(Traduora {
            client: Client::builder()
                .danger_accept_invalid_certs(!self.validate_certs)
                .build()?,
            rest_url: self.build_rest_url()?,
            token: Unauthenticated,
        })
    }

    fn build_unauthenticated_async(&self) -> TraduoraResult<AsyncTraduora<Unauthenticated>> {
        Ok(AsyncTraduora {
            client: AsyncClient::builder()
                .danger_accept_invalid_certs(!self.validate_certs)
                .build()?,
            rest_url: self.build_rest_url()?,
            token: Unauthenticated,
        })
    }
}
