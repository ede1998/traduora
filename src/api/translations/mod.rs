//! Contains all endpoints under path `/api/v1/projects/{projectId}/translations`.

mod common;
mod list;

pub use common::*;
pub use list::Translations;
