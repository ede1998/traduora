//! Contains all endpoints under path `/api/v1/locales`

mod list;

pub use list::{AllLocales, Locale, LocaleCode};
