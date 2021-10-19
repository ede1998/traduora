use async_trait::async_trait;

use crate::api::{ApiError, AsyncClient, Client};

/// A trait which represents a query which may be made to a Traduora client.
///
/// This is the more general version of [crate::Query] because it allows the caller
/// of the trait to chose the type to deserialize to.
/// The distinction is useful to prevent deserialization of fields that the caller is not
/// interested in or to allow deserialization when the Traduora instance returns an
/// unexpected model.
///
/// # Examples
/// ```no_run
/// use serde::Deserialize;
/// use traduora::{CustomQuery, TraduoraError, api::users::Me};
///
/// #[derive(Deserialize)]
/// struct IdOnlyInfo {
///     id: String,
/// }
///
/// # fn main() -> Result<(), TraduoraError> {
/// # let client = traduora::api::doctests::DummyClient;
/// let user_info: IdOnlyInfo = Me.query_custom(&client)?;
/// assert!(!user_info.id.is_empty());
/// # Ok(())
/// # }
/// ```
pub trait CustomQuery<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn query_custom(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a Traduora client.
///
/// This is the more general version of [crate::AsyncQuery] because it allows the caller
/// of the trait to chose the type to deserialize to.
/// The distinction is useful to prevent deserialization of fields that the caller is not
/// interested in or to allow deserialization when the Traduora instance returns an
/// unexpected model.
///
/// # Examples
/// ```no_run
/// use serde::Deserialize;
/// use traduora::{AsyncCustomQuery, TraduoraError, api::users::Me};
///
/// #[derive(Deserialize)]
/// struct IdOnlyInfo {
///     id: String,
/// }
///
/// # async fn main_async() -> Result<(), TraduoraError> {
/// # let client = traduora::api::doctests::DummyClient;
/// let user_info: IdOnlyInfo = Me.query_custom_async(&client).await?;
/// assert!(!user_info.id.is_empty());
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait AsyncCustomQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn query_custom_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
