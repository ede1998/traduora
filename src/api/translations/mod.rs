//! Contains all endpoints under path `/api/v1/projects/{projectId}/translations`.

mod common;
mod delete;
mod edit;
mod list;

pub use common::*;
pub use delete::DeleteLocale;
pub use edit::EditTranslation;
pub use list::Translations;
