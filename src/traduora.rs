use std::convert::TryInto;
use std::fmt::{self, Debug};

use async_trait::async_trait;
use bytes::Bytes;
use http::Response as HttpResponse;
use log::{debug, error};
use thiserror::Error;
use url::Url;

use crate::api;
use crate::auth::{AuthError, Authenticated, Scope, Unauthenticated};
use crate::{ApiError, AsyncClient, AsyncQuery, Client, Login, Query, RestClient};

/// The error type which is returned by constructor for a Traduora client.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TraduoraError {
    /// URL for the Traduora API failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// Inner error.
        #[from]
        source: url::ParseError,
    },
    /// Authorization header could not be set.
    #[error("error setting auth header: {}", source)]
    AuthError {
        /// Inner error.
        #[from]
        source: AuthError,
    },
    /// Reqwest failed to process the request.
    #[error("communication with traduora: {}", source)]
    Communication {
        /// Inner error.
        #[from]
        source: reqwest::Error,
    },
    /// HTTP error.
    #[error("traduora HTTP error: {}", status)]
    Http {
        /// Status code returned from server
        status: reqwest::StatusCode,
    },
    /// No response from Traduora.
    #[error("no response from traduora")]
    NoResponse {},
    /// Serde failed to deserialize the JSON to the given type.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// Inner error.
        #[source]
        source: serde_json::Error,
        /// The type that failed to deserialize.
        typename: &'static str,
    },
    /// Error accessing the API.
    #[error("api error: {}", source)]
    Api {
        /// Inner error.
        #[from]
        source: ApiError<RestError>,
    },
}

type TraduoraResult<T> = Result<T, TraduoraError>;

/// A representation of the Traduora API for a single user.
///
/// Separate users should use separate instances of this.
#[derive(Clone)]
pub struct Traduora<A: Scope> {
    /// The client to use for API calls.
    client: reqwest::blocking::Client,
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
    ///
    /// Calling this method does not query the API.
    ///
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.

    /// # Examples
    /// ```
    /// # use traduora::TraduoraError;
    /// use traduora::Traduora;
    /// # fn main() -> Result<(), TraduoraError> {
    /// let client = Traduora::new("localhost:8080")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).build()
    }

    /// Create a new non-SSL Traduora API representation.
    ///
    /// Calling this method does not query the API.
    ///
    /// # Warning
    /// It is **strongly** recommended to use [`Traduora::new`] instead to force encryption.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```
    /// # use traduora::TraduoraError;
    /// use traduora::Traduora;
    /// # fn main() -> Result<(), TraduoraError> {
    /// let client = Traduora::new_insecure("localhost:8080")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_insecure<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .build()
    }

    /// Tries to authenticate the Traduora client.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Errors
    /// This method returns an error if the provided credentials are invalid.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, Traduora};
    /// # fn main() -> Result<(), TraduoraError> {
    /// let client = Traduora::new_insecure("localhost:8080")?;
    /// let login = Login::password("user@traduora.example", "password");
    /// let authenticated_client = client.authenticate(&login)?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Create a new Traduora API representation and authenticate
    /// the user.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, Traduora};
    /// # fn main() -> Result<(), TraduoraError> {
    /// let login = Login::password("user@traduora.example", "password");
    /// let client = Traduora::with_auth("localhost:8080", login)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_auth<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).authenticate(login).build()
    }

    /// Create a new non-SSL Traduora API representation
    /// and authenticate the user.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Warning
    /// It is **strongly** recommended to use [`Traduora::new`] instead to force encryption.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, Traduora};
    /// # fn main() -> Result<(), TraduoraError> {
    /// let login = Login::password("user@traduora.example", "password");
    /// let client = Traduora::with_auth_insecure("localhost:8080", login)?;
    /// # Ok(())
    /// # }
    /// ```
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

impl<A: Scope> RestClient for Traduora<A> {
    type Error = RestError;
    type AccessLevel = A;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        debug!(target: "traduora", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl<A: Scope> Client for Traduora<A> {
    fn rest(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
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
        call().map_err(ApiError::client)
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
impl<A: Scope> RestClient for AsyncTraduora<A> {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        debug!(target: "traduora", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }

    type AccessLevel = Authenticated;
}

#[async_trait]
impl<A: Scope + Send + Sync> AsyncClient for AsyncTraduora<A> {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
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
        call().await.map_err(ApiError::client)
    }
}

impl AsyncTraduora<Unauthenticated> {
    /// Create a new Traduora API representation.
    ///
    /// Calling this method does not query the API.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```
    /// # use traduora::TraduoraError;
    /// use traduora::AsyncTraduora;
    /// # fn main() -> Result<(), TraduoraError> {
    /// let client = AsyncTraduora::new("localhost:8080")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref()).build_async()
    }

    /// Create a new non-SSL Traduora API representation.
    ///
    /// Calling this method does not query the API.
    ///
    /// # Warning
    /// It is **strongly** recommended to use [`AsyncTraduora::new`] instead to force encryption.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```
    /// # use traduora::TraduoraError;
    /// use traduora::AsyncTraduora;
    /// # fn main() -> Result<(), TraduoraError> {
    /// let client = AsyncTraduora::new_insecure("localhost:8080")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_insecure<T>(host: T) -> TraduoraResult<Self>
    where
        T: AsRef<str>,
    {
        Builder::new(host.as_ref())
            .use_http(true)
            .validate_certs(false)
            .build_async()
    }

    /// Tries to authenticate the Traduora client.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Errors
    /// This method returns an error if the provided credentials are invalid.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, AsyncTraduora};
    /// # async fn main_async() -> Result<(), TraduoraError> {
    /// let client = AsyncTraduora::new_insecure("localhost:8080")?;
    /// let login = Login::password("user@traduora.example", "password");
    /// let authenticated_client = client.authenticate(&login).await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Create a new Traduora API representation
    /// and authenticate the user.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, AsyncTraduora};
    /// # async fn main_async() -> Result<(), TraduoraError> {
    /// let login = Login::password("user@traduora.example", "password");
    /// let client = AsyncTraduora::with_auth("localhost:8080", login).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_auth<T>(host: T, login: Login) -> TraduoraResult<Self>
    where
        T: AsRef<str> + Sync + Send + 'static,
    {
        Builder::new(host.as_ref())
            .authenticate(login)
            .build_async()
            .await
    }

    /// Create a new non-SSL Traduora API representation
    /// and authenticate the user.
    ///
    /// Calling this method queries the Traduora API.
    ///
    /// # Warning
    /// It is **strongly** recommended to use [`Traduora::new`] instead to force encryption.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    ///
    /// # Examples
    /// ```no_run
    /// # use traduora::TraduoraError;
    /// use traduora::{Login, AsyncTraduora};
    /// # async fn main_async() -> Result<(), TraduoraError> {
    /// let login = Login::password("user@traduora.example", "password");
    /// let client = AsyncTraduora::with_auth_insecure("localhost:8080", login).await?;
    /// # Ok(())
    /// # }
    /// ```
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

/// Creates a new instance of [`Traduora`] or [`AsyncTraduora`] with custom parameters.
///
/// The builder is what the constructors on these types call under the hood.
///
/// # Examples
/// Assume you want to connect to a Traduora instance with encryption
/// that only has a self-signed certificate and you already stored an
/// access token somewhere.
/// ```
/// use traduora::{api::AccessToken, TraduoraBuilder};
///
/// # fn main() -> Result<(), traduora::TraduoraError> {
/// let token = AccessToken::new("eyJhbGc...................XMywm-zM");
/// let client = TraduoraBuilder::new("localhost:8080")
///     .use_http(true)
///     .validate_certs(false)
///     .with_access_token(token)
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug)]
#[must_use]
pub struct Builder<'h, L> {
    host: &'h str,
    protocol: &'static str,
    validate_certs: bool,
    login: L,
}

impl<'h> Builder<'h, ()> {
    /// Construct a new builder instance.
    ///
    /// The builder is intialized with the following defaults:
    /// - uses HTTPS
    /// - validates certificates
    /// - unauthenticated access
    pub const fn new(host: &'h str) -> Self {
        Self {
            host,
            protocol: "https",
            validate_certs: true,
            login: (),
        }
    }

    /// Adds login information to the builder.
    ///
    /// Note that the Traduora API is not queried when calling this
    /// function. It is queried only when calling [`Builder::build`]
    /// or [`Builder::build_async`].
    pub const fn authenticate(self, login: Login) -> Builder<'h, Login> {
        Builder {
            host: self.host,
            protocol: self.protocol,
            validate_certs: self.validate_certs,
            login,
        }
    }

    /// Adds an access token string to the builder.
    ///
    /// Note that the Traduora API won't be queried at all when the
    /// client is built with this method. The token is assumed to be valid
    /// and passed to the client without any modifications.
    pub fn with_access_token(
        self,
        token: impl Into<api::AccessToken>,
    ) -> Builder<'h, api::AccessToken> {
        Builder {
            host: self.host,
            protocol: self.protocol,
            validate_certs: self.validate_certs,
            login: token.into(),
        }
    }

    /// Builds a synchronous client without authentification information.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    pub fn build(&self) -> TraduoraResult<Traduora<Unauthenticated>> {
        self.build_unauthenticated()
    }

    /// Builds an asynchronous client without authentification information.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    pub fn build_async(&self) -> TraduoraResult<AsyncTraduora<Unauthenticated>> {
        self.build_unauthenticated_async()
    }
}

impl<'h> Builder<'h, Login> {
    /// Builds a synchronous client with authentification information.
    ///
    /// Calling this method queries the Traduora API for an access token.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    pub fn build(self) -> TraduoraResult<Traduora<Authenticated>> {
        let api = self.build_unauthenticated()?;
        api.authenticate(&self.login)
    }

    /// Builds an asynchronous client with authentification information.
    ///
    /// Calling this method queries the Traduora API for an access token.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the provided credentials are invalid.
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
    pub async fn build_async(self) -> TraduoraResult<AsyncTraduora<Authenticated>> {
        let api = self.build_unauthenticated_async()?;
        api.authenticate(&self.login).await
    }
}

impl<'h> Builder<'h, api::AccessToken> {
    /// Builds a synchronous client with authentification information.
    ///
    /// Calling this method does not query the Traduora API. The access
    /// token is assumed to be valid. In case it's not, calls to endpoints
    /// requiring authentification will fail.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::blocking::Client`] cannot be initialized.
    pub fn build(&self) -> TraduoraResult<Traduora<Authenticated>> {
        let api = self.build_unauthenticated()?;
        Ok(Traduora {
            client: api.client,
            rest_url: api.rest_url,
            token: self.login.clone().into(),
        })
    }

    /// Builds an asynchronous client with authentification information.
    ///
    /// Calling this method does not query the Traduora API. The access
    /// token is assumed to be valid. In case it's not, calls to endpoints
    /// requiring authentification will fail.
    ///
    /// # Errors
    /// This method returns an error if
    /// - the host url fails to parse.
    /// - the underlying [`reqwest::Client`] cannot be initialized.
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
    /// Decides whether to connect with unencrypted HTTP
    /// or via HTTPS.
    ///
    /// # Warning
    /// It is **strongly** recommended to use encryption. Otherwise,
    /// login data will be sent in plain text.
    /// You can try a self-signed certificate instead, or even better
    /// a fully valid one.
    pub const fn use_http(mut self, use_http: bool) -> Self {
        self.protocol = if use_http { "http" } else { "https" };
        self
    }

    /// Decides whether the SSL certificates will be validate when
    /// opening the connection.
    ///
    /// # Warning
    /// It is recommended to just use valid (non-self-signed) certificates.
    pub const fn validate_certs(mut self, validate: bool) -> Self {
        self.validate_certs = validate;
        self
    }

    fn build_rest_url(&self) -> Result<Url, url::ParseError> {
        format!("{}://{}/api/v1/", self.protocol, self.host).parse()
    }

    fn build_unauthenticated(&self) -> TraduoraResult<Traduora<Unauthenticated>> {
        Ok(Traduora {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(!self.validate_certs)
                .build()?,
            rest_url: self.build_rest_url()?,
            token: Unauthenticated,
        })
    }

    fn build_unauthenticated_async(&self) -> TraduoraResult<AsyncTraduora<Unauthenticated>> {
        Ok(AsyncTraduora {
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(!self.validate_certs)
                .build()?,
            rest_url: self.build_rest_url()?,
            token: Unauthenticated,
        })
    }
}
