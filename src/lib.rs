pub mod api;
pub mod auth;
mod query;
mod traduora;

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
