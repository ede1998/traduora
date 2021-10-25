//! Contains all endpoints under path `/api/v1/projects/{projectId}/terms`.

mod common;
mod create;
mod delete;
mod edit;
mod list;

pub use common::*;
pub use create::CreateTerm;
pub use delete::DeleteTerm;
pub use edit::EditTerm;
pub use list::Terms;
