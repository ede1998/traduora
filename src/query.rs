//! See type level explanations, especially [`Query`] or [`AsyncQuery`].

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::endpoint;
use crate::{ApiError, AsyncClient, AsyncCustomQuery, Client, CustomQuery, Endpoint};

/// This trait defines the type that an endpoint
/// should deserialize to by default.
pub trait DefaultModel: Endpoint {
    type Model: DeserializeOwned;

    /// This mapping function parses a [`Value`] to [`Self::Model`].
    ///
    /// The default implementation parses the value into a model that is wrapped
    /// in a "data" object. Most Traduora endpoints return their answer in this
    /// form. A notable exception is [`crate::api::auth::AccessToken`].
    fn map(data: Value) -> Result<Self::Model, serde_json::Error> {
        #[derive(Deserialize)]
        #[serde(bound = "T: DeserializeOwned")]
        struct Container<T> {
            data: T,
        }
        serde_json::from_value::<Container<Self::Model>>(data).map(|h| h.data)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DefaultQuery<'e, E> {
    endpoint: &'e E,
}

impl<'e, E, C> CustomQuery<E::Model, C> for DefaultQuery<'e, E>
where
    E: DefaultModel,
    C: Client,
    E::AccessControl: From<C::AccessLevel>,
{
    fn query_custom(&self, client: &C) -> Result<E::Model, ApiError<C::Error>> {
        let (req, data) = endpoint::build_request_with_body(self.endpoint, client)?;
        let rsp = client.rest(req, data)?;
        endpoint::process_response(&rsp, E::map)
    }
}

#[async_trait]
impl<'e, E, C> AsyncCustomQuery<E::Model, C> for DefaultQuery<'e, E>
where
    E: DefaultModel + Sync,
    C: AsyncClient + Sync,
    E::AccessControl: From<C::AccessLevel>,
{
    async fn query_custom_async(&self, client: &C) -> Result<E::Model, ApiError<C::Error>> {
        let (req, data) = endpoint::build_request_with_body(self.endpoint, client)?;
        let rsp = client.rest_async(req, data).await?;
        endpoint::process_response(&rsp, E::map)
    }
}

/// A trait which represents a query which may be made to a Traduora client.
///
/// The returned model should be a full representation of the data that this
/// endpoint can return. For more fine-grained control of the deserialized model,
/// see [`CustomQuery`].
pub trait Query<C>: DefaultModel
where
    C: Client,
{
    /// Perform the query against the client.
    ///
    /// # Errors
    /// This method returns an error if
    /// - fails to prepare the request.
    /// - the request could not be sent to the server.
    /// - the server returns a non-success status code.
    /// - the returned JSON fails to deserialize.
    fn query(&self, client: &C) -> Result<Self::Model, ApiError<C::Error>>;
}

/// A trait which represents a asynchronous query which may be made to a Traduora client.
///
/// The returned model should be a full representation of the data that this
/// endpoint can return. For more fine-grained control of the deserialized model,
/// see [`AsyncCustomQuery`].
#[async_trait]
pub trait AsyncQuery<C>: DefaultModel
where
    C: AsyncClient,
{
    /// Perform the query against the client asynchronously.
    ///
    /// # Errors
    /// This method returns an error if
    /// - fails to prepare the request.
    /// - the request could not be sent to the server.
    /// - the server returns a non-success status code.
    /// - the returned JSON fails to deserialize.
    async fn query_async(&self, client: &C) -> Result<Self::Model, ApiError<C::Error>>;
}

impl<C, E> Query<C> for E
where
    C: Client,
    E: DefaultModel,
    E::AccessControl: From<C::AccessLevel>,
{
    fn query(&self, client: &C) -> Result<Self::Model, ApiError<C::Error>> {
        DefaultQuery { endpoint: self }.query_custom(client)
    }
}

#[async_trait]
impl<C, E> AsyncQuery<C> for E
where
    C: AsyncClient + Sync,
    E: DefaultModel + Sync,
    E::AccessControl: From<C::AccessLevel>,
{
    async fn query_async(&self, client: &C) -> Result<Self::Model, ApiError<C::Error>> {
        DefaultQuery { endpoint: self }
            .query_custom_async(client)
            .await
    }
}
