#![allow(clippy::module_name_repetitions)]

use std::path::PathBuf;

use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum BilalError {
    #[error("No such file {0:?}")]
    NoFile(PathBuf),

    #[error("Invalid config")]
    InvalidConfig { source: toml::de::Error },

    #[error("No such method {0:?}")]
    InvalidMethod(String),

    #[error("No such madhab {0:?}")]
    InvalidMadhab(String),

    // All cases of `std::io::Error`.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
