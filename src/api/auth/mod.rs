//! Contains all endpoints under path `/api/v1/auth`

mod providers;
mod signup;
mod token;

pub use providers::{AuthProvider, Providers};
pub use signup::{NewUser, Signup};
pub use token::{AccessToken, Token};
