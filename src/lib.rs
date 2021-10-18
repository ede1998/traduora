pub mod api;
pub mod auth;
mod query;
mod traduora;

/// Alias for [`AuthentificateRequest`](api::auth::token::AuthentificateRequest).
/// The shorter and clearer name improves readability when
/// building a [`Traduora`] or [`AsyncTraduora`] client.
pub type Login = api::auth::token::AuthentificateRequest;

pub use query::{AsyncQuery, Query};
pub use traduora::AsyncTraduora;
pub use traduora::Builder as TraduoraBuilder;
pub use traduora::Traduora;
pub use traduora::TraduoraError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
