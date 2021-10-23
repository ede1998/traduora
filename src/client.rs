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
    use crate::{auth::Authenticated, traduora::RestError, ApiError};

    use super::RestClient;

    /// A dummy client to use in doc tests.
    ///
    /// The implementation is not functional.
    /// The doc tests should be annotated with `no_run`.
    #[doc(hidden)]
    pub struct DummyClient;

    impl Client for DummyClient {
        fn rest(
            &self,
            _: RequestBuilder,
            _: Vec<u8>,
        ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
            Ok(Response::builder().body(Bytes::new()).unwrap())
        }
    }

    impl RestClient for DummyClient {
        type Error = RestError;

        type AccessLevel = Authenticated;

        fn rest_endpoint(&self, _: &str) -> Result<reqwest::Url, ApiError<Self::Error>> {
            Ok("https:://www.traduora.example".parse()?)
        }
    }

    #[async_trait]
    impl AsyncClient for DummyClient {
        async fn rest_async(
            &self,
            _: RequestBuilder,
            _: Vec<u8>,
        ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
            Ok(Response::builder().body(Bytes::new()).unwrap())
        }
    }
}
