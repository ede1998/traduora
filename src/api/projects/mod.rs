//! Contains all project-related endpoints under path `/api/v1/projects`.

mod common;
mod create;
mod list;
mod show;

pub use common::*;
pub use create::CreateProject;
pub use list::Projects;
pub use show::ShowProject;
