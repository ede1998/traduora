pub mod api;
pub mod auth;
mod fetch;
mod traduora;

pub use fetch::{AsyncFetcher, Fetcher};
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
