use std::path::PathBuf;

use miette::{Diagnostic, NamedSource, SourceOffset};
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("Invalid configuration: {message}")]
    #[diagnostic(
        code(bilal::invalid_config),
        url(docsrs),
        help("See the configuration example https://github.com/azzamsa/bilal#usage-examples")
    )]
    InvalidConfig {
        #[source_code]
        src: NamedSource<String>,
        #[label("{message}")]
        bad_bit: SourceOffset,
        message: String,
    },

    #[error("No such method {0:?}")]
    InvalidMethod(String),

    #[error("No such madhab {0:?}")]
    InvalidMadhab(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("Configuration file is not found in `{path}`")]
    #[diagnostic(
        code(bilal::no_config),
        url(docsrs),
        help("Try creating a config file. See https://github.com/azzamsa/bilal#usage-examples")
    )]
    ConfigNotFound { path: PathBuf },
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(_err: std::env::VarError) -> Self {
        Self::NotFound("env var not found".into())
    }
}

impl std::convert::From<islam::error::Error> for Error {
    fn from(err: islam::error::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
