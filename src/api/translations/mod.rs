//! Contains all endpoints under path `/api/v1/projects/{projectId}/translations`.

mod common;
mod edit;
mod list;

pub use common::*;
pub use edit::EditTranslation;
pub use list::Translations;
