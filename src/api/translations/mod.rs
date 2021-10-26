//! Contains all endpoints under path `/api/v1/projects/{projectId}/translations`.

mod common;
mod delete;
mod edit;
mod list;
mod list_locales;

pub use common::*;
pub use delete::DeleteLocale;
pub use edit::EditTranslation;
pub use list::Translations;
pub use list_locales::{Locales, ProjectLocale};
