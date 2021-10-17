pub mod api;
pub mod auth;
mod traduora;

pub use traduora::AsyncTraduora;
pub use traduora::Traduora;
pub use traduora::TraduoraError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
