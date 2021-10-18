use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::api::endpoint;
use crate::api::{ApiError, AsyncClient, AsyncCustomQuery, Client, CustomQuery, Endpoint};

pub trait DefaultModel: Endpoint {
    type Model: DeserializeOwned;

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

pub trait Query<C>: DefaultModel
where
    C: Client,
{
    fn query(&self, client: &C) -> Result<Self::Model, ApiError<C::Error>>;
}

#[async_trait]
pub trait AsyncQuery<C>: DefaultModel
where
    C: AsyncClient,
{
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
