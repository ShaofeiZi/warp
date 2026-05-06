mod catalog;
mod locale;
mod manager;
mod settings;

pub use catalog::{Catalog, I18nKey};
pub use locale::Locale;
pub use manager::I18n;
pub use settings::I18nSettings;

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod catalog_tests;
#[cfg(test)]
#[path = "locale_tests.rs"]
mod locale_tests;
#[cfg(test)]
#[path = "manager_tests.rs"]
mod manager_tests;
