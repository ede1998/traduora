//! Contains all endpoints accessible via the [Traduora API](https://docs.traduora.co/docs/api/v1/overview)
//! (that were implemented yet).

mod common;

pub mod auth;
pub mod labels;
pub mod locales;
pub mod projects;
pub mod terms;
pub mod translations;
pub mod users;

pub use common::*;
