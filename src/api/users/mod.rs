//! Contains all endpoints under path `/api/v1/auth`

mod delete;
mod me;

pub use delete::DeleteMe;
pub use me::{Me, UserInfo};
