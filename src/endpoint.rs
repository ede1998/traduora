use std::borrow::Cow;

use async_trait::async_trait;
use bytes::Bytes;
use http::{self, header, request::Builder, Method, Request, Response};
use serde::de::DeserializeOwned;

use crate::{
    auth::Scope, ApiError, AsyncClient, AsyncCustomQuery, BodyError, Client, CustomQuery,
    RestClient,
};

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// Defines the permission level that the client must have to be able to access this endpoint.
    type AccessControl: Scope;

    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    ///
    /// # Errors
    /// This method returns an error if the body could not be serialized to JSON.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E, T, C> CustomQuery<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
    E::AccessControl: From<C::AccessLevel>,
{
    fn query_custom(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let (req, data) = build_request_with_body(self, client)?;
        let rsp = client.rest(req, data)?;
        process_response(&rsp, serde_json::from_value)
    }
}

#[async_trait]
impl<E, T, C> AsyncCustomQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
    E::AccessControl: From<C::AccessLevel>,
{
    async fn query_custom_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let (req, data) = build_request_with_body(self, client)?;
        let rsp = client.rest_async(req, data).await?;
        process_response(&rsp, serde_json::from_value)
    }
}

pub fn process_response<T, E, F>(r: &Response<Bytes>, mapper: F) -> Result<T, ApiError<E>>
where
    T: DeserializeOwned,
    E: std::error::Error + Send + Sync + 'static,
    F: FnOnce(serde_json::Value) -> Result<T, serde_json::Error>,
{
    let fill = Bytes::from("null");
    let body = if r.body().is_empty() { &fill } else { r.body() };
    let result_v = serde_json::from_slice(body);
    if r.status().is_success() {
        // give general parse error or map to desired rust type or give type mapping error
        mapper(result_v?).map_err(ApiError::data_type::<T>)
    } else {
        // try to parse error as JSON or give general error
        let v = result_v.map_err(|_| ApiError::server_error(r.status(), r.body()))?;
        // give specific error message
        Err(ApiError::from_traduora(v))
    }
}

pub fn build_request_with_body<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<(Builder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    let uri = client
        .rest_endpoint(&endpoint.endpoint())?
        .as_str()
        .parse::<http::Uri>()
        .expect("failed to parse a url::Url as an http::Uri");

    let req = Request::builder().method(endpoint.method()).uri(uri);

    Ok(match endpoint.body()? {
        Some((mime, body)) => (req.header(header::CONTENT_TYPE, mime), body),
        None => (req, Vec::new()),
    })
}
