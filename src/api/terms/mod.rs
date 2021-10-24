//! Contains all endpoints under path `/api/v1/projects/{projectId}/terms`.

mod common;
mod create;
mod list;

pub use common::*;
pub use create::CreateTerm;
pub use list::Terms;
