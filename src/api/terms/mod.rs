//! Contains all endpoints under path `/api/v1/projects/{projectId}/terms`.

mod create;

pub use create::{CreateTerm, Term};
