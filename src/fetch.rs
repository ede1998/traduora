use std::error::Error;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::api::{ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query};

pub trait Fetcher: Endpoint {
    fn fetch<C, AL, AC>(&self, client: &C) -> Result<Self::Body, ApiError<C::Error>>
    where
        C: Client<AccessLevel = AL>,
        AC: From<AL>,
        Self: Sized + Fetcher<AccessControl = AC>,
    {
        let result: Value = self.query(client)?;
        map(result)
    }
}

#[async_trait]
pub trait AsyncFetcher: Endpoint {
    async fn fetch_async<C, AL, AC>(&self, client: &C) -> Result<Self::Body, ApiError<C::Error>>
    where
        C: AsyncClient<AccessLevel = AL> + Sync,
        AC: From<AL>,
        Self: Sized + Sync + AsyncFetcher<AccessControl = AC>,
    {
        let result: Value = self.query_async(client).await?;
        map(result)
    }
}

fn map<T, E>(data: Value) -> Result<T, ApiError<E>>
where
    T: DeserializeOwned,
    E: Error + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    #[serde(bound = "T: DeserializeOwned")]
    struct Container<T: DeserializeOwned> {
        data: T,
    }
    serde_json::from_value::<Container<T>>(data)
        .map(|h| h.data)
        .map_err(ApiError::data_type::<T>)
}
