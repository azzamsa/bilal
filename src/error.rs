use std::path::PathBuf;

use miette::Diagnostic;
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
        help("see the configuration example https://github.com/azzamsa/bilal#usage-examples")
    )]
    InvalidConfig { message: String },

    #[error("No such method {0:?}")]
    InvalidMethod(String),

    #[error("No such madhab {0:?}")]
    InvalidMadhab(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("configuration file is not found in `{path}`")]
    #[diagnostic(
        code(bilal::no_config),
        url(docsrs),
        help("try creating a config file. See https://github.com/azzamsa/bilal#usage-examples")
    )]
    ConfigNotFound { path: PathBuf },
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(_err: std::env::VarError) -> Self {
        Self::NotFound("env var not found".into())
    }
}

impl std::convert::From<islam::pray::error::Error> for Error {
    fn from(err: islam::pray::error::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<time::error::IndeterminateOffset> for Error {
    fn from(err: time::error::IndeterminateOffset) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<time::error::InvalidFormatDescription> for Error {
    fn from(err: time::error::InvalidFormatDescription) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<time::error::Format> for Error {
    fn from(err: time::error::Format) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
