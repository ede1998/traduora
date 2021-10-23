#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

mod client;
mod custom_query;
mod endpoint;
mod error;
#[macro_use]
mod macros;
mod query;
mod traduora;

pub mod api;
pub mod auth;

pub(crate) use client::RestClient;
pub(crate) use endpoint::Endpoint;

/// Alias for [`Token`](api::auth::Token).
/// The shorter and clearer name improves readability when
/// building a [`Traduora`] or [`AsyncTraduora`] client.
pub type Login = api::auth::Token;

pub use crate::traduora::AsyncTraduora;
pub use crate::traduora::Builder as TraduoraBuilder;
pub use crate::traduora::Traduora;
pub use crate::traduora::TraduoraError;
pub use client::{doctests::DummyClient, AsyncClient, Client};
pub use custom_query::{AsyncCustomQuery, CustomQuery};
pub use error::{ApiError, BodyError};
pub use query::{AsyncQuery, Query};
