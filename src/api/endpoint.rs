use std::borrow::Cow;

use async_trait::async_trait;
use bytes::Bytes;
use http::{self, header, request::Builder, Method, Request, Response};
use serde::de::DeserializeOwned;

use crate::{
    api::{ApiError, AsyncClient, AsyncCustomQuery, BodyError, Client, CustomQuery},
    auth::Scope,
};

use super::RestClient;

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    type AccessControl: Scope;

    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
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

pub fn process_response<T, E, F>(rsp: &Response<Bytes>, mapper: F) -> Result<T, ApiError<E>>
where
    T: DeserializeOwned,
    E: std::error::Error + Send + Sync + 'static,
    F: FnOnce(serde_json::Value) -> Result<T, serde_json::Error>,
{
    if rsp.status().is_success() {
        // try to parse as general JSON or give general parse error
        let v = serde_json::from_slice(rsp.body())?;
        // map to desired rust type or give type mapping error
        mapper(v).map_err(ApiError::data_type::<T>)
    } else {
        // try to parse error as JSON or give general error
        let v = serde_json::from_slice(rsp.body())
            .map_err(|_| ApiError::server_error(rsp.status(), rsp.body()))?;
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

/*
#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{ApiError, AsyncQuery, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug, Deserialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_traduora_non_json_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::TraduoraService {
            status, ..
        } = err
        {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_traduora_empty_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::TraduoraService {
            status, ..
        } = err
        {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_traduora_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::TraduoraService {
            status, ..
        } = err
        {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_traduora_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Traduora {
            msg,
        } = err
        {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_traduora_error_detection_legacy() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Traduora {
            msg,
        } = err
        {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_traduora_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::TraduoraUnrecognized {
            obj,
        } = err
        {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_bad_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "not_value": 0,
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType {
            source,
            typename,
        } = err
        {
            assert_eq!(format!("{}", source), "missing field `value`");
            assert_eq!(typename, "traduora::api::endpoint::tests::DummyResult");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_good_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query(&client).unwrap();
        assert_eq!(res.value, 0);
    }

    #[tokio::test]
    async fn test_good_deserialization_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query_async(&client).await.unwrap();
        assert_eq!(res.value, 0);
    }
}
 */
