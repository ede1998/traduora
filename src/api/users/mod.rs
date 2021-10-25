//! Contains all endpoints under path `/api/v1/auth`

mod delete;
mod edit;
mod me;

pub use delete::DeleteMe;
pub use edit::EditMe;
pub use me::{Me, UserInfo};
