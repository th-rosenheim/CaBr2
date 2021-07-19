#![allow(clippy::upper_case_acronyms)]

mod error;
pub mod handler;
mod types;

#[cfg(feature = "tauri_plugin")]
pub mod plugin;
#[cfg(feature = "webserver")]
pub mod webserver;

pub use handler::{get_hazard_symbols, read_config, DATA_DIR, TMP_DIR};
pub use types::{BackendConfig, GHSSymbols};
