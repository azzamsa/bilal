pub mod cli;
pub mod config;
pub mod error;
pub mod output;
pub mod prayer;

pub use error::Error;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

i18n!(
    "locales",
    fallback = "en",
    minify_key = true,
    minify_key_len = 12,
    minify_key_prefix = "t_",
    minify_key_thresh = 64
);
