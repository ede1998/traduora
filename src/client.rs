use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use url::Url;

use crate::{auth::Scope, ApiError};

/// A trait representing a client which can communicate with a Traduora instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// The permission level the client has.
    type AccessLevel: Scope;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    ///
    /// # Errors
    /// This method returns an error if it fails to concatenate the
    /// host name to the specific endpoint.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with a Traduora instance.
pub trait Client: RestClient {
    /// Send a REST query.
    ///
    /// # Errors
    /// This method returns an error if
    /// - fails to prepare the request.
    /// - the request could not be sent to the server.
    /// - the [`reqwest::Response`] could not be mapped to an [`http::Response`].
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with a Traduora instance.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    ///
    /// # Errors
    /// This method returns an error if
    /// - fails to prepare the request.
    /// - the request could not be sent to the server.
    /// - the [`reqwest::Response`] could not be mapped to an [`http::Response`].
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

#[doc(hidden)]
pub mod doctests {
    use super::*;
    use crate::{auth::Authenticated, traduora::RestError, ApiError, Login, TraduoraError};
    use http::{Method, Response};

    use super::RestClient;

    fn generate_response(method: &Method, endpoint: &str) -> Response<Bytes> {
        let is_match = |wildcard_str: &str| {
            let parts: Vec<_> = wildcard_str.split('*').collect();
            match parts.as_slice() {
                [] => endpoint.is_empty(),
                [single] => *single == endpoint,
                [first, middle @ .., last] => {
                    let mut start = 0;
                    for m in middle {
                        start = match endpoint[start..].find(m) {
                            Some(pos) => pos + m.len(),
                            None => return false,
                        };
                    }
                    endpoint.starts_with(first) && endpoint.ends_with(last)
                }
            }
        };

        let body = Bytes::from_static(match (method, endpoint) {
            (&Method::GET, "/api/v1/auth/providers") => include_bytes!("../data/providers.json"),
            (&Method::POST, "/api/v1/auth/signup") => include_bytes!("../data/signup_user.json"),
            (&Method::POST, "/api/v1/auth/token") => include_bytes!("../data/access_token.json"),
            (&Method::GET, "/api/v1/projects") => include_bytes!("../data/projects.json"),
            (&Method::POST, "/api/v1/projects") => include_bytes!("../data/create_project.json"),
            (&Method::POST, _) if is_match("/api/v1/projects/*/terms") => {
                include_bytes!("../data/new_term.json")
            }
            (&Method::GET, "/api/v1/users/me") => include_bytes!("../data/user_info.json"),
            _ => panic!(
                "Failed to find appropriate response body for {} {}",
                method, endpoint
            ),
        });

        Response::builder()
            .body(body)
            .expect("Failed to build dummy response")
    }

    /// A dummy client to use in doc tests.
    /// It does not react to inputs other than
    /// HTTP method and url and then just
    /// returns static JSON data for it.
    #[doc(hidden)]
    pub struct TestClient {
        url: String,
    }

    impl TestClient {
        /// method with same signature as normal Traduora client so we can hide it in doc tests.
        ///
        /// # Errors
        /// None, always returns ok but tries to match signature with the normal client
        pub fn new(host: &str) -> Result<Self, TraduoraError> {
            Ok(Self { url: host.into() })
        }

        /// method with same signature as normal Traduora client so we can hide it in doc tests.
        ///
        /// # Errors
        /// None, always returns ok but tries to match signature with the normal client
        pub fn with_auth(host: &str, _: Login) -> Result<Self, TraduoraError> {
            Ok(Self { url: host.into() })
        }
    }

    impl Client for TestClient {
        fn rest(
            &self,
            builder: RequestBuilder,
            _: Vec<u8>,
        ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
            let request = builder.body(()).map_err(|e| ApiError::client(e.into()))?;
            Ok(generate_response(request.method(), request.uri().path()))
        }
    }

    impl RestClient for TestClient {
        type Error = RestError;

        type AccessLevel = Authenticated;

        fn rest_endpoint(&self, endpoint: &str) -> Result<reqwest::Url, ApiError<Self::Error>> {
            Ok(format!("http://{}/api/v1/{}", self.url, endpoint).parse()?)
        }
    }

    #[async_trait]
    impl AsyncClient for TestClient {
        async fn rest_async(
            &self,
            builder: RequestBuilder,
            _: Vec<u8>,
        ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
            let request = builder.body(()).map_err(|e| ApiError::client(e.into()))?;
            Ok(generate_response(request.method(), request.uri().path()))
        }
    }
}
