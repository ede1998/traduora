//! Contains all endpoints under path `/api/v1/auth`.

mod change_password;
mod providers;
mod signup;
mod token;

pub use change_password::ChangePassword;
pub use providers::{AuthProvider, Providers};
pub use signup::{NewUser, Signup};
pub use token::{AccessToken, Token};
