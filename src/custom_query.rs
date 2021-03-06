use async_trait::async_trait;

use crate::{ApiError, AsyncClient, Client};

/// A trait which represents a query which may be made to a Traduora client.
///
/// This is the more general version of [`crate::Query`] because it allows the caller
/// of the trait to chose the type to deserialize to.
/// The distinction is useful to prevent deserialization of fields that the caller is not
/// interested in or to allow deserialization when the Traduora instance returns an
/// unexpected model.
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use serde::Deserialize;
/// use traduora::{CustomQuery, api::users::Me};
///
/// #[derive(Deserialize)]
/// struct UserDataInfo {
///     data: IdOnlyInfo,
/// }
///
/// #[derive(Deserialize)]
/// struct IdOnlyInfo {
///     id: String,
/// }
///
/// # let login = Login::password("user@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let user_info: UserDataInfo = Me.query_custom(&client)?;
///
/// assert_eq!(user_info.data.id, "40379230-ced0-43b8-8b78-37c924f491a7");
/// # Ok::<(), TraduoraError>(())
/// ```
pub trait CustomQuery<T, C>
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
    fn query_custom(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a Traduora client.
///
/// This is the more general version of [`crate::AsyncQuery`] because it allows the caller
/// of the trait to chose the type to deserialize to.
/// The distinction is useful to prevent deserialization of fields that the caller is not
/// interested in or to allow deserialization when the Traduora instance returns an
/// unexpected model.
///
/// # Examples
/// ```
/// # use traduora::{Login, TestClient as Traduora, TraduoraError};
/// use serde::Deserialize;
/// use traduora::{AsyncCustomQuery, api::users::Me};
///
/// #[derive(Deserialize)]
/// struct UserDataInfo {
///     data: IdOnlyInfo,
/// }
///
/// #[derive(Deserialize)]
/// struct IdOnlyInfo {
///     id: String,
/// }
///
/// # async fn main_async() -> Result<(), TraduoraError> {
/// # let login = Login::password("user@mail.example", "letmeinpls");
/// let client = Traduora::with_auth("localhost:8080", login)?;
/// let user_info: UserDataInfo = Me.query_custom_async(&client).await?;
///
/// assert_eq!(user_info.data.id, "40379230-ced0-43b8-8b78-37c924f491a7");
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait AsyncCustomQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    ///  
    /// # Errors
    /// This method returns an error if
    /// - fails to prepare the request.
    /// - the request could not be sent to the server.
    /// - the server returns a non-success status code.
    /// - the returned JSON fails to deserialize.
    async fn query_custom_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
