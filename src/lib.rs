#![doc = include_str!("../README.md")]

pub mod api;
pub mod auth;
mod query;
mod traduora;

/// Alias for [`Token`](api::auth::Token).
/// The shorter and clearer name improves readability when
/// building a [`Traduora`] or [`AsyncTraduora`] client.
pub type Login = api::auth::Token;

pub use crate::traduora::AsyncTraduora;
pub use crate::traduora::Builder as TraduoraBuilder;
pub use crate::traduora::Traduora;
pub use crate::traduora::TraduoraError;
pub use api::{AsyncCustomQuery, CustomQuery};
pub use query::{AsyncQuery, Query};
