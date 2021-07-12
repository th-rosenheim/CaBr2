#![allow(clippy::new_without_default)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::upper_case_acronyms)]

mod error;
pub mod handler;
pub mod types;

#[cfg(feature = "gestis")]
mod gestis;

#[cfg(feature = "tauri_plugin")]
pub mod plugin;
#[cfg(feature = "wasm")]
pub mod wasm;
