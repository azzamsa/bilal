pub mod cli;
pub mod config;
pub mod error;
pub mod output;
pub mod prayer;

pub use error::Error;

// Use internal type. Chrono API changes very often
pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::NaiveDateTime;
