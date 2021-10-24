//! Contains all project-related endpoints under path `/api/v1/projects`.

mod list;

pub use list::{Project, Projects};
